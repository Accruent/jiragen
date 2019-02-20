use clap::Arg;
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;

pub static DEFAULT_ISSUES_FILE_PATH: &str = "./jiragen_issues.csv";
pub static DEFAULT_CONFIG_FILE_PATH: &str = "./jiragen.json";

/// JiraGen configuration that is read from/saved to the config file.
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
  pub jira_url: String,
  pub jira_user: String,
  pub jira_password: String,
}

/// Returns the --config argument options
pub fn get_config_arg<'a, 'b>() -> Arg<'a, 'b> {
  Arg::with_name("config")
    .long("config")
    .short("c")
    .help("A custom path to the config file.")
    .default_value(DEFAULT_CONFIG_FILE_PATH)
    .takes_value(true)
}

/// Returns the --issues argument options
pub fn get_issues_arg<'a, 'b>() -> Arg<'a, 'b> {
  Arg::with_name("issues")
    .long("issues")
    .short("i")
    .help("A custom path to the issues template CSV file")
    .default_value(DEFAULT_ISSUES_FILE_PATH)
    .takes_value(true)
}

/// Reads the contents of the given file path and returns a JiraGen Config object
pub fn read_config_file(path: &str) -> Config {
  let config_str = &read_to_string(path).unwrap();
  let json = serde_json::from_str(config_str).unwrap();

  json
}
