use crate::config::{get_config_arg, write_config_str, Config};
use clap::{App, ArgMatches, SubCommand};
use std::fs::write;

/// Returns the `init` SubCommand
pub fn subcommand_init<'a, 'b>() -> App<'a, 'b> {
  SubCommand::with_name("init")
    .about("Initializes the JiraGen config and issues template.")
    .visible_alias("i")
    .display_order(10)
    .arg(get_config_arg())
}

/// Processes the `init` SubCommand.
/// Creates the config file as well as the issues template file.
pub fn parse_init(matches: &ArgMatches) {
  // write config
  let config_path = matches.value_of("config").unwrap();
  let config = Config {
    jira_url: "",
    jira_user: "",
    jira_password: "",
    issues_file_path: None,
  };
  let config_yaml_str = write_config_str(&config);

  write(config_path, config_yaml_str).unwrap();
  println!("Wrote config: {}", config_path);

  // write issues template
}
