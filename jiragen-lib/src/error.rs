use csv::Error as csvError;
use failure::Fail;
use reqwest::Error as reqwestError;
use serde_json::Error as jsonError;
use std::io::Error as ioError;

#[derive(Debug, Fail)]
#[fail(display = "An error occurred: {}\n\nDetails:\n{}", message, details)]
/// Used as a possible value of `Error`.
pub struct CustomError {
  pub message: String,
  pub details: String,
}

#[derive(Debug, Fail)]
/// Error handler for JiraGen-related errors. Encapsulates CSV, Reqwest, File, and Serialization errors.
pub enum Error {
  #[fail(display = "An error occurred when parsing csv file: {}", _0)]
  CsvError(#[fail(cause)] csvError),
  #[fail(display = "An error occurred when sending request: {}", _0)]
  ReqError(#[fail(cause)] reqwestError),
  #[fail(display = "An error occurred during file operation: {}", _0)]
  IoError(#[fail(cause)] ioError),
  #[fail(display = "An error occurred when parsing json: {}", _0)]
  JsonError(#[fail(cause)] jsonError),
  #[fail(display = "An error occurred: {}", _0)]
  CustomError(CustomError),
}

impl From<csvError> for Error {
  fn from(e: csvError) -> Self {
    Error::CsvError(e)
  }
}

impl From<ioError> for Error {
  fn from(e: ioError) -> Self {
    Error::IoError(e)
  }
}

impl From<jsonError> for Error {
  fn from(e: jsonError) -> Self {
    Error::JsonError(e)
  }
}

impl From<reqwestError> for Error {
  fn from(e: reqwestError) -> Self {
    Error::ReqError(e)
  }
}
