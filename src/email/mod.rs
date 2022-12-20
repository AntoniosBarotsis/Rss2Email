//! Constructs and sends emails from different providers.

#[allow(clippy::use_self)]
#[allow(clippy::module_name_repetitions)]
pub mod email_provider;
pub mod mail_cmd;
pub mod sendgrid;
pub mod error;

/// Holds all environment variables that are required
/// by any email provider.
#[derive(Debug)]
pub struct EnvLoader {
  pub(crate) api_key: Option<String>,
}

impl EnvLoader {
  /// Creates a new `EnvLoader` by loading the
  /// `API_KEY` environment variable.
  pub(crate) fn new() -> Self {
    Self {
      api_key: std::env::var("API_KEY").ok(),
    }
  }
}
