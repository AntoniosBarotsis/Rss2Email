use super::sendgrid::SendGrid;
use log::warn;

/// An email provider abstraction to allow for multiple backends.
pub trait EmailProvider {
  /// Sends an email to and from the specified address.
  fn send_email(&self, address: &str, api_key: &str, contents: &str);
}

/// Abstracts away the email backend.
///
/// Currently only Sendgrid is implemented but in the future,
/// the implementation choice will be made here.
///
/// The env arg is trimmed and converted to uppercase for the
/// sake of consistency before matching it.
///
/// By default, this will return the `SendGrid` implementation.
/// 
/// New `EmailProvider` implementations must be added here, otherwise
/// they will be ignored.
pub fn get_email_provider() -> impl EmailProvider {
  let env_var = std::env::var("EMAIL")
    .ok()
    .unwrap_or_else(|| "SENDGRID".to_owned());

  #[allow(clippy::match_single_binding)]
  match env_var.trim().to_uppercase().as_str() {
    "SENDGRID" => SendGrid::new(),
    e => {
      warn!("Invalid Email provider: {}, defaulting to SendGrid.", e);
      SendGrid::new()
    }
  }
}
