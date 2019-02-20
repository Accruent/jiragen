#[macro_use]
extern crate lazy_static;

mod api;
mod config;
mod error;
mod init;
mod push;
mod serialize;

use clap::{crate_authors, crate_version, App, AppSettings};
// use generate::{parse_generate, subcommand_generate};
use init::{parse_init, subcommand_init};
use push::{parse_push, subcommand_push};

fn main() {
  let matches = App::new("JiraGen")
    .about("A CLI tool to generate JIRA issues and place them on a board.")
    .bin_name("jiragen")
    .version(crate_version!())
    .author(crate_authors!())
    .setting(AppSettings::SubcommandRequiredElseHelp)
    .setting(AppSettings::DisableHelpSubcommand)
    .subcommand(subcommand_init())
    .subcommand(subcommand_push())
    .get_matches();

  match matches.subcommand() {
    ("init", Some(cmd)) => {
      match parse_init(cmd) {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
      };
    }
    // ("generate", Some(cmd)) => parse_generate(cmd),
    ("push", Some(cmd)) => {
      match parse_push(cmd) {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
      };
    }
    _ => println!("Invalid command"),
  }
}
