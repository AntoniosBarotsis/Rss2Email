use log::info;

use super::{email_provider::EmailProvider, EmailError, EnvLoader};

#[derive(Default)]
/// `EmailProvider` implementation using [`SendGrid`](https://sendgrid.com/).
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

    let contents = contents.replace('\"', "\\\"");
    let message = format!(
      r#"{{"personalizations": [{{"to": [{{"email": "{address}"}}]}}],"from": {{"email": "{address}"}},"subject": "Rss2Email","content": [{{"type": "text/html", "value": "{contents}"}}]}}"#
    );

    let req = ureq::post("https://api.sendgrid.com/v3/mail/send")
      .set("Authorization", &format!("Bearer {}", api_key))
      .set("Content-Type", "application/json")
      .send_string(&message);

    match req {
      Ok(req) => {
        info!(
          "Email request sent with {} {}",
          req.status(),
          req.status_text()
        );
        Ok(())
      }
      Err(e) => Err(e.into()),
    }
  }
}
