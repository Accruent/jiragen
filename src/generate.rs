use crate::config::get_config_arg;
use clap::{App, ArgMatches, SubCommand};
// use std::fs::{read, write};

/// Returns the `generate` SubCommand.
pub fn subcommand_generate<'a, 'b>() -> App<'a, 'b> {
  SubCommand::with_name("generate")
    .about("Generate the issues from the issues template file into the JIRA project.")
    .visible_alias("g")
    .display_order(20)
    .arg(get_config_arg())
}

/// Processes the `generate` Subcommand.
/// Parses the issues template and generates corresponding issues in JIRA.
pub fn parse_generate(matches: &ArgMatches) {
  let config_path = matches.value_of("config").unwrap();
}
