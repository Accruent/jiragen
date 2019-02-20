/// This file contains functionality specific to interacting with the JIRA API.
use crate::config::Config;
use http::Method;
use reqwest::{header::HeaderMap, Client, RequestBuilder};
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

// /// A tuple that represents the (id, name) attributes of a JIRA field.
// pub type IssueField = (String, String);

/// Handles requests sent to JIRA.
pub struct JiraClient {
  pub client: Client,
  pub config: Config,
}

impl JiraClient {
  /// Creates a new `reqwest` client and returns the `JiraClient` struct wrapper.
  pub fn new(config: Config) -> Self {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let client = Client::builder().default_headers(headers).build().unwrap();

    Self { client, config }
  }

  /// Creates a reqwest Request Builder with some predefined authorization headers
  pub fn init_request(&self, method_str: &str, endpoint: &str) -> RequestBuilder {
    let method_str_ucase = method_str.to_uppercase();
    let method = method_str_ucase.as_bytes();
    // let method = match

    let url = format!("{}{}", self.config.jira_url, endpoint);
    self
      .client
      .request(Method::from_bytes(method).unwrap(), &url)
      .basic_auth(&self.config.jira_user, Some(&self.config.jira_password))
  }
}

// #[derive(Debug, Serialize)]
// #[serde(untagged)]
/// The different types of issue field shapes that can be sent to JIRA:
/// 1. An object with string: string keys:values.
/// 1. An array of strings.
/// 1. An array of #1 above.
// pub enum JiraIssueFieldVariant {
//   StringHashMap(HashMap<String, String>),
//   StringVec(Vec<String>),
//   StringHashMapVec(Vec<HashMap<String, String>>),
// }

#[derive(Debug, Serialize)]
/// The object to send to JIRA’s "bulk issue creation" API endpoint
pub struct JiraIssue {
  pub update: Option<HashMap<String, HashMap<String, Vec<String>>>>, // not used, default 'NONE'
  pub fields: Value,
}
