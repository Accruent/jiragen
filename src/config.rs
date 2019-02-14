pub static DEFAULT_ISSUES_FILE_PATH: &str = "./jiragen_issues.csv";
pub static DEFAULT_CONFIG_FILE_PATH: &str = "./jiragen.yaml";

pub struct Config<'a> {
  pub jira_url: &'a str,
  pub jira_user: &'a str,
  pub jira_password: &'a str,
  pub issues_file_path: Option<&'a str>,
}

pub fn write_config_str(config: &Config) -> String {
  let config_str = format!(
    r#"jira_url = "{}"
jira_user = "{}"
jira_password = "{}"
issues_file_path = "{}"
"#,
    config.jira_url,
    config.jira_user,
    config.jira_password,
    match config.issues_file_path {
      Some(issues_file_path) => issues_file_path,
      None => DEFAULT_ISSUES_FILE_PATH,
    }
  );

  config_str
}

pub fn get_default_issues_template() -> &'static str {
  static default_issues_template: &str = r#""#;

  default_issues_template
}
