// use strict_yaml_rust::{StrictYamlEmitter, StrictYamlLoader};

mod config;
mod generate;
mod init;

use clap::{crate_authors, crate_version, App, AppSettings};
use generate::{parse_generate, subcommand_generate};
use init::{parse_init, subcommand_init};

fn main() {
  let matches = App::new("JiraGen")
    .about("A CLI tool to generate JIRA issues and place them on a board.")
    .bin_name("jiragen")
    .version(crate_version!())
    .author(crate_authors!())
    .setting(AppSettings::SubcommandRequiredElseHelp)
    .setting(AppSettings::DisableHelpSubcommand)
    .subcommand(subcommand_init())
    .subcommand(subcommand_generate())
    .get_matches();

  match matches.subcommand() {
    ("init", Some(cmd)) => parse_init(cmd),
    ("generate", Some(cmd)) => parse_generate(cmd),
    _ => println!("Invalid command"),
  }
}
