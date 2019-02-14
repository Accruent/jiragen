use clap::{crate_authors, crate_version, App, AppSettings, Arg, ArgMatches, SubCommand};
// use strict_yaml_rust::{StrictYamlEmitter, StrictYamlLoader};
use std::fs::{read, write};

mod config;
use config::{write_config_str, Config};

fn main() {
  let matches = App::new("JiraGen")
    .about("A CLI tool to generate JIRA issues and place them on a board.")
    .bin_name("jiragen")
    .version(crate_version!())
    .author(crate_authors!())
    .setting(AppSettings::SubcommandRequiredElseHelp)
    .setting(AppSettings::DisableHelpSubcommand)
    .subcommand(
      SubCommand::with_name("init")
        .about("Initializes the JiraGen config and issues template.")
        .visible_alias("i")
        .display_order(10)
        .arg(get_config_arg()),
    )
    .subcommand(
      SubCommand::with_name("generate")
        .about("Generate the issues from the issues template file into the JIRA project.")
        .visible_alias("g")
        .display_order(20)
        .arg(get_config_arg()),
    )
    .get_matches();

  match matches.subcommand() {
    ("init", Some(cmd)) => parse_init(cmd),
    ("generate", Some(cmd)) => parse_generate(cmd),
    _ => println!("Invalid command"),
  }
}

fn get_config_arg<'a, 'b>() -> Arg<'a, 'b> {
  Arg::with_name("config")
    .long("config")
    .short("c")
    .help("A custom path to the config file (default \"./jiragen.yaml\").")
    .default_value("./jiragen.yaml")
    .takes_value(true)
}

fn parse_init(matches: &ArgMatches) {
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

fn parse_generate(matches: &ArgMatches) {
  let config_path = matches.value_of("config").unwrap();
}
