use crate::config::{get_config_arg, get_issues_arg, Config};
use crate::error::Error;
use clap::{App, ArgMatches, SubCommand};
use serde_json::json;
use std::fs::write;

/// Returns the `init` SubCommand.
pub fn subcommand_init<'a, 'b>() -> App<'a, 'b> {
  SubCommand::with_name("init")
    .about("Creates the JiraGen config file.")
    .visible_alias("i")
    .display_order(10)
    .arg(get_config_arg())
    .arg(get_issues_arg())
}

/// Processes the `init` SubCommand.
/// Creates the config file as well as the issues template file.
pub fn parse_init(matches: &ArgMatches) -> Result<(), Error> {
  // write config
  let config_path = matches.value_of("config").unwrap();
  let issues_path = matches.value_of("issues").unwrap();

  let config = Config {
    jira_url: "".to_string(),
    jira_user: "".to_string(),
    jira_password: "".to_string(),
    issues_schema: json!({
      "project": { "key": "string" },
      "summary": "string",
      "description": "string",
      "issuetype": {"id": "string"},
      "labels": ["string"],
      "assignee": {"name": "string"},
    }),
  };

  write(config_path, serde_json::to_string_pretty(&config)?)?;
  println!("Wrote config: {}", config_path);

  let mut csv_writer = csv::Writer::from_path(issues_path)?;
  csv_writer.write_record(&[
    "Project",
    "Summary",
    "Description",
    "Issue Type",
    "Labels",
    "Assignee",
  ])?;
  csv_writer.write_record(&[
    "project.key",
    "summary",
    "description",
    "issuetype.id",
    "labels[]",
    "assignee.name",
  ])?;

  println!("Wrote issues: {}", issues_path);

  Ok(())
}