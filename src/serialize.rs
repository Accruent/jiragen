// use crate::config::Config;
use crate::error::Error;
use csv::StringRecord;
use regex::Regex;
use serde_json::{Map, Value};

/// Reads the issues template .csv file and converts it to a JSON object
/// ```
/// extern crate filesystem;
/// ```
pub fn csv_to_json(headers: Vec<&str>, records: Vec<StringRecord>) -> Result<Vec<Value>, Error> {
  let arr: Vec<Value> = records
    .iter()
    .map(|record| {
      let mut fields = Map::new();

      for (i, id) in headers.iter().enumerate() {
        let record_field_value: &str = match record.get(i) {
          Some(value) => value,
          None => "",
        };

        let (json_key, json_val) = csv_value_to_json(id, record_field_value);

        match fields.contains_key(&json_key) {
          true => {
            fields = merge_json(fields, json_key, json_val);
          }
          false => {
            fields.insert(json_key, json_val);
          }
        };
      }

      Value::Object(fields)
    })
    .collect();

  Ok(arr)
}

// Converts the supplied header column string (which describes the json schema) and value into a JSON object.
fn csv_value_to_json(header: &str, value: &str) -> (String, Value) {
  lazy_static! {
    static ref FIND_NESTING_DELIMITERS: Regex = Regex::new(r"\[\]|\.").unwrap();
  }

  let val: Value = match FIND_NESTING_DELIMITERS.find(header) {
    Some(matching_delimiter) => {
      // get the string following the "[]" delimiter. If empty string, then we know that the value is an array of strings
      let rest_header: &str = &header[matching_delimiter.end()..];

      match matching_delimiter.as_str() {
        // is array
        "[]" => {
          let nested_val = match rest_header {
            // an array of strings
            "" => Value::Array(vec![Value::String(value.to_string())]),
            // an array of object|arrays
            _ => {
              let (_, recursive_val) = csv_value_to_json(rest_header, value);
              Value::Array(vec![recursive_val])
            }
          };

          nested_val
        }

        // is object
        "." => {
          let nested_val = match rest_header {
            // This should never happen. There should always be a non-empty string following the `.`
            "" => panic!("There should always be text following the period (.)"),
            // an object of strings|objects|arrays
            _ => {
              let (recursive_key, recursive_val) = csv_value_to_json(rest_header, value);
              let mut fields = Map::new();
              fields.insert(recursive_key, recursive_val);
              Value::Object(fields)
            }
          };

          nested_val
        }
        _ => panic!("csv_value_to_json() matched a character that should not have been matched."),
      }
    }
    None => Value::String(value.to_string()),
  };

  let root_header_name: Vec<&str> = FIND_NESTING_DELIMITERS.split(header).collect();

  (root_header_name[0].to_string(), val)
}

// Merges json_val into fields, where fields already has a value for json_key.
// If json_val is an object, then the key:val is appended to the existing object.
// If json_val is an array, then each item in the array is appended to the existing array.
fn merge_json(
  mut fields: Map<String, Value>,
  json_key: String,
  json_val: Value,
) -> Map<String, Value> {
  let existing_val = fields.get_mut(&json_key).unwrap();
  match existing_val {
    Value::Object(existing_map) => {
      // append key:vals to the existing object
      if let Value::Object(json_val_map) = json_val {
        for (key, val) in json_val_map.into_iter() {
          existing_map.insert(key, val);
        }
      }
    }

    Value::Array(existing_vec) => {
      if let Value::Array(json_val_vec) = json_val {
        for val in json_val_vec.into_iter() {
          existing_vec.push(val);
        }
      }
    }

    _ => {
      // assumed to be a string
      fields.insert(json_key, json_val);
    }
  };

  fields
}

// /// Reads the config `issues_schema` and converts it to CSV headers
// pub fn json_schema_to_csv_headers(schema: &Value) {
//   // read each key/Value
//   // if string, great
//   // if array, get first value in array, recurse
//   // if object, recurse with object,
//   // need to also send the path that was taken to form the final syntax
// }

#[cfg(test)]
mod tests {
  use super::*;
  use serde_json::json;

  #[test]
  fn csv_to_json_converts_string() {
    let headers = vec!["summary"];
    let records = vec![StringRecord::from(vec!["A Test Summary"])];

    assert_eq!(
      csv_to_json(headers, records).unwrap(),
      vec![json!({ "summary": "A Test Summary" })]
    );
  }

  #[test]
  fn csv_to_json_converts_string_array() {
    let headers = vec!["labels[]"];
    let records = vec![StringRecord::from(vec!["a-label"])];

    assert_eq!(
      csv_to_json(headers, records).unwrap(),
      vec![json!({ "labels": ["a-label"] })]
    );
  }

  #[test]
  fn csv_to_json_converts_multiple_string_array() {
    let headers = vec!["labels[]", "labels[]"];
    let records = vec![StringRecord::from(vec!["a-label", "b-label"])];

    assert_eq!(
      csv_to_json(headers, records).unwrap(),
      vec![json!({ "labels": ["a-label", "b-label"] })]
    );
  }

  #[test]
  fn csv_to_json_converts_string_obj_property() {
    let headers = vec!["issuetype.id"];
    let records = vec![StringRecord::from(vec!["12345"])];

    assert_eq!(
      csv_to_json(headers, records).unwrap(),
      vec![json!({ "issuetype": {"id": "12345"} })]
    );
  }

  #[test]
  fn csv_to_json_converts_array_obj() {
    let headers = vec!["components[].id"];
    let records = vec![StringRecord::from(vec!["A Component"])];

    assert_eq!(
      csv_to_json(headers, records).unwrap(),
      vec![json!({ "components": [{"id": "A Component"}] })]
    );
  }

  #[test]
  fn csv_to_json_converts_nested_array_obj() {
    let headers = vec!["watcher.watchers[].accountId"];
    let records = vec![StringRecord::from(vec!["abcc281-qk3j8d8fj"])];

    assert_eq!(
      csv_to_json(headers, records).unwrap(),
      vec![json!({
        "watcher": {
          "watchers": [
            {"accountId": "abcc281-qk3j8d8fj"}
          ]
        }
      })]
    );
  }

  #[test]
  fn csv_to_json_converts_multiple_obj_properties() {
    let headers = vec![
      "timetracking.originalEstimate",
      "timetracking.remainingEstimate",
    ];
    let records = vec![StringRecord::from(vec!["10", "5"])];

    assert_eq!(
      csv_to_json(headers, records).unwrap(),
      vec![json!({ "timetracking": { "originalEstimate": "10", "remainingEstimate": "5" } })]
    );
  }

  #[test]
  fn csv_to_json_converts_multiple_array_items() {
    let headers = vec!["fixVersions[].id", "fixVersions[].id"];
    let records = vec![StringRecord::from(vec!["10000", "10001"])];

    assert_eq!(
      csv_to_json(headers, records).unwrap(),
      vec![json!({ "fixVersions": [ {"id": "10000"}, {"id": "10001"} ] })]
    );
  }
}
