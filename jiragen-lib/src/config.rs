use serde::{Deserialize, Serialize};

/// JiraGen configuration that is used for sending requests to JIRA.
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
  pub jira_url: String,
  pub jira_user: String,
  pub jira_password: String,
}
