use std::fmt::Display;

/// Represents all things that could go wrong
/// while trying to send an email.
#[derive(Debug)]
#[allow(dead_code)]
pub enum EmailError {
  Config(String),
  Request(reqwest::Error),
  Io(String),
  Other(String),
}

impl From<reqwest::Error> for EmailError {
  fn from(e: reqwest::Error) -> Self {
    Self::Request(e)
  }
}

impl From<resend_rs::Error> for EmailError {
  fn from(value: resend_rs::Error) -> Self {
    match value {
      resend_rs::Error::Http(e) => Self::from(e),
      resend_rs::Error::Resend(e) => Self::Other(e.to_string()),
    }
  }
}

impl Display for EmailError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match &self {
      Self::Request(e) => write!(f, "{e}"),
      Self::Config(e) | Self::Io(e) | Self::Other(e) => write!(f, "{e}"),
    }
  }
}
