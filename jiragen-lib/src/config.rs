use serde::{Deserialize, Serialize};

/// JiraGen configuration that is used for sending requests to JIRA. The username and password fields are required because JiraGen uses Basic Authentication to send requests to JIRA.
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
  /// The URL of the JIRA server.
  pub jira_url: String,
  /// The user name to login to JIRA
  pub jira_user: String,
  /// The user password to login to JIRA
  pub jira_password: String,
}
