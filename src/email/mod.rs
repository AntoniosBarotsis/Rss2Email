use std::fmt::Display;

#[allow(clippy::module_name_repetitions)]
#[allow(clippy::use_self)]
pub mod email_provider;
pub mod mail_cmd;
pub mod sendgrid;

/// Holds all environment variables that are required
/// by any email provider.
pub struct EnvLoader {
  pub(crate) api_key: Option<String>,
}

impl EnvLoader {
  pub(crate) fn new() -> Self {
    Self {
      api_key: std::env::var("API_KEY").ok(),
    }
  }
}

#[allow(dead_code)]
/// Represents all things that could go wrong
/// while trying to send an email.
pub enum EmailError {
  Config(String),
  Request(Box<reqwest::Error>),
  Io(String),
  Other(String),
}

impl From<reqwest::Error> for EmailError {
  fn from(e: reqwest::Error) -> Self {
    Self::Request(Box::new(e))
  }
}

impl Display for EmailError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match &self {
      Self::Request(e) => write!(f, "{}", e),
      Self::Config(e) | Self::Io(e) | Self::Other(e) => write!(f, "{}", e),
    }
  }
}
