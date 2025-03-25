//! [`EmailProvider`] implementation using [`Resend`](https://resend.com/).

use resend_rs::types::CreateEmailBaseOptions;

use crate::info;

use super::{email_provider::EmailProvider, error::EmailError, EnvLoader};

#[derive(Default, Debug)]
pub struct Resend {
  api_key: Option<String>,
}

impl Resend {
  pub(crate) fn new(env_loader: &EnvLoader) -> Self {
    Self {
      api_key: env_loader.api_key.clone(),
    }
  }
}

impl EmailProvider for Resend {
  fn send_email(
    &self,
    from_address: &str,
    recipient_addresses: Vec<&str>,
    subject: &str,
    contents: &str,
  ) -> Result<(), EmailError> {
    let api_key = self
      .api_key
      .as_ref()
      .ok_or_else(|| EmailError::Config("Cannot use Resend without API_KEY".to_owned()))
      .cloned()?;

    let resend = resend_rs::Resend::new(&api_key);

    let email =
      CreateEmailBaseOptions::new(from_address, recipient_addresses, subject).with_html(contents);

    match resend.emails.send(email) {
      Ok(_id) => {
        info!("Email request sent");
      }
      Err(e) => return Err(EmailError::from(e)),
    }

    Ok(())
  }
}
