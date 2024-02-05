use std::env;

use rss2email_lib::email::email_provider::{EmailProvider, EmailProviders};

#[test]
fn load_sendgrid() {
  env::remove_var("API_KEY");

  let sendgrid =
    EmailProviders::try_from("SENDGRID".to_owned()).expect("The Sendgrid provider is defined");

  assert!(
    sendgrid
      .send_email("address", vec!["person"], "subject", "email")
      .is_err(),
    "Mandatory API_KEY should cause an Err()"
  );
  env::set_var("API_KEY", "ASD");
  assert!(
    sendgrid
      .send_email("address", vec!["person"], "subject", "email")
      .is_err(),
    "Failed to load proper Email Provider SendGrid"
  );
  env::remove_var("API_KEY");
}
