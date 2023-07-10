//! Constructs and sends emails from different providers.

#[allow(clippy::use_self)]
#[allow(clippy::module_name_repetitions)]
pub mod email_provider;
pub mod error;
pub mod mail_cmd;
pub mod resend;
pub mod sendgrid;

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
