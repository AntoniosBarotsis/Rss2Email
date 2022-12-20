//! An email provider abstraction to allow for multiple backends.

use super::{error::EmailError, sendgrid::SendGrid};
use super::{mail_cmd::MailCommand, EnvLoader};
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait EmailProvider {
  /// Sends an email to and from the specified address.
  fn send_email(&self, address: &str, contents: &str) -> Result<(), EmailError>;
}

/// An enum containing all Email Provider implementations.
#[derive(Debug)]
#[enum_dispatch(EmailProvider)]
pub enum EmailProviders {
  SendGrid(SendGrid),
  MailCommand(MailCommand),
}

/// Abstracts away the email backend.
///
/// The email provider is picked by inspecting the
/// `EMAIL` environment variable.
///
/// By default, this will return the `SendGrid` implementation.
pub fn get_email_provider() -> Result<impl EmailProvider, String> {
  let env_var = std::env::var("EMAIL")
    .ok()
    .unwrap_or_else(|| "SENDGRID".to_owned());

  EmailProviders::try_from(env_var)
}

impl TryFrom<String> for EmailProviders {
  type Error = String;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    let client = value.trim().to_uppercase();
    let env_vars = EnvLoader::new();

    match client.as_str() {
      "SENDGRID" => Ok(Self::SendGrid(SendGrid::new(&env_vars))),
      "MAIL_COMMAND" => Ok(Self::MailCommand(MailCommand {})),
      _ => Err("Requested client not found".to_owned()),
    }
  }
}
