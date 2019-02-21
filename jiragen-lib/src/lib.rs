//! `jiragen` is a collection of utilities for sending requests to JIRA. It includes functionality for sending bulk issue creation from a .csv file.
//! ```no_run
//! use csv::{Reader, StringRecord};
//! use jiragen::{Config, csv_to_json, JiraClient, JiraIssue};
//! use serde_json::json;
//!
//! // Configure & initialize JIRA client
//! let config = Config {
//!   jira_url: "my-jira.com".to_string(),
//!   jira_user: "user".to_string(),
//!   jira_password: "password".to_string(),
//! };
//! let jira = JiraClient::new(config);
//!
//! // Prepare CSV data
//! let mut csv_reader = Reader::from_path("jiragen_issues.csv").unwrap();
//! let ids_record = csv_reader.headers().unwrap().clone();
//! let ids: Vec<&str> = ids_record.iter().collect();
//!
//! let mut csv_records = csv_reader.into_records();
//! csv_records.next(); // skip line 2, which contains human-readable field names
//!
//! // create bulk issues to send starting on line 3
//! let filtered_csv_records: Vec<StringRecord> = csv_records
//!   .filter_map(|record_result| match record_result {
//!     Ok(result) => Some(result),
//!     Err(_) => None,
//!   })
//!   .collect();
//! let json_values = csv_to_json(ids, filtered_csv_records).unwrap();
//!
//! let issues_to_create: Vec<JiraIssue> = json_values
//!   .into_iter()
//!   .map(|record_json| JiraIssue {
//!     update: None,
//!     fields: record_json,
//!   })
//!   .collect();
//!
//! let bulk_issue_create_request = jira.init_request("POST", "/rest/api/2/issue/bulk");
//! let request_json = json!({ "issueUpdates": issues_to_create });
//!
//! let mut response = bulk_issue_create_request
//!   .body(request_json.to_string())
//!   .send();
//! ```

#[macro_use]
extern crate lazy_static;

mod api;
pub use api::{JiraClient, JiraIssue};

mod config;
pub use config::Config;

mod error;
pub use error::{CustomError, Error};

mod serialize;
pub use serialize::csv_to_json;
