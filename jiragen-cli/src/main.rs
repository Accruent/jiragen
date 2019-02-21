//! The command line tool to send bulk issue creation requests to JIRA from a .csv file.
//!
//! ## Installation
//!
//! Download the binary (located in the releases section of the GitHub repo) and run it on the command line. Alternatively if you already have Rust installed, you can run `cargo install jiragen-cli`.
//!
//! ## Usage
//!
//! ```bash
//! A CLI tool to generate JIRA issues and place them on a board.
//!
//! USAGE:
//!     jiragen <SUBCOMMAND>
//!
//! FLAGS:
//!     -h, --help       Prints help information
//!     -V, --version    Prints version information
//!
//! SUBCOMMANDS:
//!     init    Creates the JiraGen config file. [aliases: i]
//!     push    Pushes issues to JIRA. [aliases: p]
//! ```
//!
//! 1. Run `jiragen init`. This creates a config file (default: `./jiragen.json`) and issues template file (default: //! `./jiragen_issues.csv`).
//! 1. Edit the config file with your JIRA credentials and save the file.
//! 1. Edit the issues template .csv file with the issues you would like to generate. Feel free to remove any field //! columns that are not necessary for issue creation. See [section about how to enter column data](#csv-syntax).
//! 1. Run `jiragen push`. This reads the data in the file and creates the corresponding issues in JIRA.
//!
//! ## Commands
//!
//! ### Command: `jiragen init`
//!
//! Creates the JiraGen config file.
//!
//! ```bash
//! jiragen init
//! #=> creates jiragen.json
//!
//! jiragen init --config ./config/my-custom-jiragen-config.json
//! #=> creates "./config/my-custom-jiragen-config.json"
//!
//! jiragen init --config ./config/my-custom-jiragen-config.json  --issues ./config/my-issues-template.csv
//! #=> creates "./config/my-custom-jiragen-config.json" and "./config/my-issues-template.csv"
//! ```
//!
//! ### Command: `jiragen push`
//!
//! Takes the content from the issues template file and creates the issues in the JIRA project.
//!
//! ```bash
//! jiragen push
//! #=> reads jiragen-issues.csv in the current folder and pushes issues to JIRA
//!
//! jiragen push --config ./config/my-custom-jiragen-config.json --issues ./config/my-issues-template.csv
//! #=> reads the files located at "./config/my-custom-jiragen-config.json" and "./config/my-issues-template.csv" //! and pushes issues to JIRA
//! ```
//!
//! ### Command Options
//!
//! **`--config`** (default: `"./jiragen.json"`)
//! A custom path where the config file is created.
//!
//! **`--issues`** (default: `"./jiragen-issues.csv"`)
//! A custom path where the issues template CSV file is created.
//!
//! ## Configuration
//!
//! Configuration is stored in a `.json` file (default `./jiragen.json`) and has the following properties:
//!
//! **`jira_url`** (string)
//! The URL of the Jira instance.
//!
//! **`jira_user`** (string)
//! The JIRA user to login as.
//!
//! **`jira_password`** (string)
//! The JIRA user’s password. (The tool uses Basic Auth).
//!
//! ## .csv syntax
//!
//! Because JIRA’s API requires that the issues’ fields be shaped to specific schemas (See [JIRA’s API](https://developer.atlassian.com/cloud/jira/platform/rest/v2/#api-api-2-issue-bulk-post) for an example), we translate that schema to the .csv file: `[]` and `.` describe an array or object property, respectively. Remember that the second row of the .csv file is ignored.
//!
//! Some examples of how data is converted from the .csv file to JSON:
//!
//! ```bash
//! # first row = csv id field/key, second row = readable field name (ignored), third row = value of that id
//!
//! summary
//! Summary # Ignored
//! A Test Summary
//! # { "summary": "A Test Summary" }
//!
//! labels[]
//! Labels # Ignored
//! a-label
//! # { "labels": ["a-label"] }
//!
//! issuetype.id
//! Issue Type # Ignored
//! 12345
//! # { "issuetype": {"id": "12345"} }
//!
//! components[].id
//! Components # Ignored
//! A Component
//! # { "components": [ {"id": "A Component"} ] }
//!
//! watcher.watchers[].accountId
//! Watchers # Ignored
//! abcc281-qk3j8d8fj
//! # { "watcher": { "watchers": [{"accountId": "abcc281-qk3j8d8fj"}] } }
//!
//! timetracking.originalEstimate,timetracking.remainingEstimate
//! Time Tracking,Time Tracking # Ignored
//! 10,5
//! # { "timetracking": { "originalEstimate": "10", "remainingEstimate": "5" } }
//!
//! fixVersions[].id,fixVersions[].id
//! Fix Versions,Fix Versions # Ignored
//! 10000,10001
//! # { "fixVersions": [ {"id": "10000"}, {"id": "10001"} ] }
//! ```

mod config;
mod init;
mod push;

use clap::{crate_authors, crate_version, App, AppSettings};
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
