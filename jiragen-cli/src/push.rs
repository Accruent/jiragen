use crate::config::{get_config_arg, get_issues_arg, read_config_file};
use clap::{App, ArgMatches, SubCommand};
use csv::{Reader, StringRecord};
use jiragen::{csv_to_json, CustomError, Error, JiraClient, JiraIssue};
use serde_json::{json, Value};

/// Returns the `push` SubCommand.
pub fn subcommand_push<'a, 'b>() -> App<'a, 'b> {
  SubCommand::with_name("push")
    .about("Pushes issues to JIRA.")
    .visible_alias("p")
    .display_order(30)
    .arg(get_config_arg())
    .arg(get_issues_arg())
}

/// Processes the `push` SubCommand.
/// Parses the issues template file and creates corresponding issues in JIRA.
pub fn parse_push(matches: &ArgMatches) -> Result<(), Error> {
  let config_path = matches.value_of("config").unwrap();
  let issues_path = matches.value_of("issues").unwrap();

  let config = read_config_file(config_path);
  let jira = JiraClient::new(config);

  let mut csv_reader = Reader::from_path(&issues_path).unwrap();

  let ids_record = csv_reader.headers()?.clone();
  let ids: Vec<&str> = ids_record.iter().collect();
  let mut csv_records = csv_reader.into_records();

  // skip line 2, which contains human-readable field names
  csv_records.next();

  // create bulk issues to send starting on line 3
  let filtered_csv_records: Vec<StringRecord> = csv_records
    .filter_map(|record_result| match record_result {
      Ok(result) => Some(result),
      Err(_) => None,
    })
    .collect();
  let json_values = csv_to_json(ids, filtered_csv_records)?;

  let issues_to_create: Vec<JiraIssue> = json_values
    .into_iter()
    .map(|record_json| JiraIssue {
      update: None,
      fields: record_json,
    })
    .collect();

  let bulk_issue_create_request = jira.init_request("POST", "/rest/api/2/issue/bulk");
  let request_json = json!({ "issueUpdates": issues_to_create });

  let mut response = bulk_issue_create_request
    .body(request_json.to_string())
    .send()?;
  let response_json: Value = response.json()?;

  if !&response.status().is_success() {
    return Err(Error::CustomError(CustomError {
      message: format!(
        "JIRA responded with status {}:",
        &response.status().as_str()
      ),
      details: serde_json::to_string_pretty(&response_json)?,
    }));
  }

  println!(
    "Issues created successfully. Response:\n\n{}",
    serde_json::to_string_pretty(&response_json)?
  );

  Ok(())
}
