use super::mail_cmd::MailCommand;
use super::sendgrid::SendGrid;
use enum_dispatch::enum_dispatch;

/// An email provider abstraction to allow for multiple backends.
#[enum_dispatch]
pub trait EmailProvider {
  /// Sends an email to and from the specified address.
  fn send_email(&self, address: &str, contents: &str);
}

/// An enum containing all Email Provider implementations.
#[enum_dispatch(EmailProvider)]
enum EmailProviders {
  SendGrid(SendGrid),
  MailCommand(MailCommand),
}

impl TryFrom<String> for EmailProviders {
  type Error = String;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    let client = value.trim().to_uppercase();
    match client.as_str() {
      "SENDGRID" => {
        let api_key = std::env::var("API_KEY");
        if let Ok(the_key) = api_key {
          Ok(Self::SendGrid(SendGrid::new(the_key)))
        } else {
          Err("Cannot use SendGrid without API_KEY".to_owned())
        }
      }
      "MAIL_COMMAND" => Ok(Self::MailCommand(MailCommand {})),
      _ => Err("Requested client not found".to_owned()),
    }
  }
}

/// Abstracts away the email backend.
///
/// By default, this will return the `SendGrid` implementation.
pub fn get_email_provider() -> Result<impl EmailProvider, String> {
  let env_var = std::env::var("EMAIL")
    .ok()
    .unwrap_or_else(|| "SENDGRID".to_owned());

  EmailProviders::try_from(env_var)
}

#[cfg(test)]
mod tests {

  use super::EmailProviders;
  use std::env;

  #[test]
  fn load_sendgrid() {
    env::remove_var("API_KEY");
    assert!(
      EmailProviders::try_from("SENDGRID".to_owned()).is_err(),
      "Mandatory API_KEY should cause an Err()"
    );
    env::set_var("API_KEY", "ASD");
    assert!(
      EmailProviders::try_from("SENDGRID".to_owned()).is_ok(),
      "Failed to load proper Email Provider SendGrid"
    );
    env::remove_var("API_KEY");
  }
}
