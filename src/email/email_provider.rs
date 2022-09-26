use super::sendgrid::SendGrid;
use enum_dispatch::enum_dispatch;
use log::warn;

/// An email provider abstraction to allow for multiple backends.
#[enum_dispatch]
pub trait EmailProvider {
  /// Sends an email to and from the specified address.
  fn send_email(&self, address: &str, api_key: &str, contents: &str);
}

/// An enum containing all Email Provider implementations.
#[enum_dispatch(EmailProvider)]
enum EmailProviders {
  SendGrid(SendGrid),
}

impl From<String> for EmailProviders {
  fn from(input: String) -> Self {
    // Note that the input is trimmed and converted
    // to upper case for the sake of consistency
    match input.trim().to_uppercase().as_str() {
      "SENDGRID" => Self::SendGrid(SendGrid::new()),
      e => {
        warn!("Invalid Email provider: {}, defaulting to SendGrid.", e);
        Self::SendGrid(SendGrid::new())
      }
    }
  }
}

/// Abstracts away the email backend.
///
/// By default, this will return the `SendGrid` implementation.
pub fn get_email_provider() -> impl EmailProvider {
  let env_var = std::env::var("EMAIL")
    .ok()
    .unwrap_or_else(|| "SENDGRID".to_owned());

  EmailProviders::from(env_var)
}
