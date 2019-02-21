/// Originally this module was going to be used to generate a csv with all of the field columns available, however I realized that doing that would require serializing & deserializing all of the different schemas that JIRA has (and are largely undocumented). We are scrapping this functionality for now.

use crate::api::{IssueField, JiraClient};
use crate::config::{get_config_arg, get_issues_arg, read_config_file, DEFAULT_ISSUE_FIELDS};
use clap::{App, ArgMatches, SubCommand};
use reqwest::Result;
use serde_json::value::Value;
use std::cmp::Ordering;

/// Returns the `generate` SubCommand.
pub fn subcommand_generate<'a, 'b>() -> App<'a, 'b> {
  SubCommand::with_name("generate")
    .about("Creates the issues template .csv file.")
    .visible_alias("g")
    .display_order(20)
    .arg(get_config_arg())
    .arg(get_issues_arg())
}

/// Processes the `generate` Subcommand.
/// Creates the issues template file.
pub fn parse_generate(matches: &ArgMatches) {
  let config_path = matches.value_of("config").unwrap();
  let issues_path = matches.value_of("issues").unwrap();

  let config = read_config_file(config_path);
  let jira = JiraClient::new(config);
  let mut fields = match get_issue_fields(&jira) {
    Ok(issue_fields) => issue_fields,
    Err(err) => {
      eprintln!(
        "An error occurred when retrieving field names from JIRA. Using default fields. Error: {}",
        err
      );

      get_default_issue_fields_vec()
    }
  };
  fields = sort_fields(fields);
  let field_ids: Vec<&str> = fields.iter().map(|(id, _)| id.as_str()).collect();
  let field_names: Vec<&str> = fields.iter().map(|(_, name)| name.as_str()).collect();

  let mut csv_writer = csv::Writer::from_path(issues_path).unwrap();
  csv_writer.write_record(field_ids).unwrap();
  csv_writer.write_record(field_names).unwrap();
}

// Returns a vector of string tuples `(jira field id, jira field name)` representing the fields in JIRA.
fn get_issue_fields(jira: &JiraClient) -> Result<Vec<IssueField>> {
  let mut response = jira.init_request("GET", "/rest/api/2/field/").send()?;
  let status = response.status();

  if !status.is_success() {
    eprintln!("An error occurred when retrieving field names from JIRA. Using default fields. Status code: {}",
    status);

    return Ok(get_default_issue_fields_vec());
  }

  let json_raw: Vec<Value> = response.json()?;

  Ok(json_raw.iter().map(field_to_string).collect())
}

// Converts a JIRA field response object to a tuple of Strings representing (id, name) of the field
fn field_to_string(field: &Value) -> IssueField {
  (
    field["id"].as_str().or(Some("")).unwrap().to_string(),
    field["name"].as_str().or(Some("")).unwrap().to_string(),
  )
}

// returns the default set of fields used for issue template creation
fn get_default_issue_fields_vec() -> Vec<IssueField> {
  DEFAULT_ISSUE_FIELDS
    .iter()
    .map(|(id, name)| (id.to_string(), name.to_string()))
    .collect()
}

// Reorders the fields returned from the API by the configured `issues_fields` first, and then alphabetically thereafter.
fn sort_fields(mut fields: Vec<IssueField>) -> Vec<IssueField> {
  // sort fields by alphabetical order
  fields.sort_unstable_by(|a, b| {
    if a.1 == b.1 {
      return Ordering::Equal;
    }

    match a.1 < b.1 {
      true => Ordering::Less,
      false => Ordering::Greater,
    }
  });

  let mut default_field_names: Vec<(IssueField)> = DEFAULT_ISSUE_FIELDS
    .iter()
    .map(|(id, name)| (id.to_string(), name.to_string()))
    .collect();

  // remove the matching fields that already exists in configuration
  let filtered_fields: Vec<IssueField> = fields
    .into_iter()
    .filter_map(|(id, name)| {
      match default_field_names
        .iter()
        .position(|(_, default_name)| default_name == &name)
      {
        // found a  => remove
        Some(_) => None,
        None => Some((id, name)),
      }
    })
    .collect();

  // then append filtered api fields to configured issues
  default_field_names.extend(filtered_fields);
  default_field_names
}
