//! [`EmailProvider`] implementation using [`SendGrid`](https://sendgrid.com/).

use crate::info;

use super::{email_provider::EmailProvider, error::EmailError, EnvLoader};

#[derive(Default, Debug)]
pub struct SendGrid {
  api_key: Option<String>,
}

impl SendGrid {
  pub(crate) fn new(env_loader: &EnvLoader) -> Self {
    Self {
      api_key: env_loader.api_key.clone(),
    }
  }
}

impl EmailProvider for SendGrid {
  fn send_email(&self, address: &str, contents: &str) -> Result<(), EmailError> {
    let api_key = self
      .api_key
      .as_ref()
      .ok_or_else(|| EmailError::Config("Cannot use SendGrid without API_KEY".to_owned()))?;

    let message = format!(
      r#"{{"personalizations": [{{"to": [{{"email": "{address}"}}]}}],"from": {{"email": "{address}"}},"subject": "Rss2Email","content": [{{"type": "text/html", "value": "{contents}"}}]}}"#
    );

    let http_client = reqwest::blocking::Client::new();
    let req = http_client
      .post("https://api.sendgrid.com/v3/mail/send")
      .header("Authorization", &format!("Bearer {api_key}"))
      .header("Content-Type", "application/json")
      .body(message)
      .build()?;
    let response = http_client.execute(req);

    match response {
      Ok(response) => {
        info!("Email request sent with {}", response.status().as_str());
        Ok(())
      }
      Err(e) => Err(e.into()),
    }
  }
}
