/**
 * Implementation for default "mail" command in linux
 */
use super::email_provider::EmailProvider;

#[derive(Default)]
pub struct MailCommand {}

impl EmailProvider for MailCommand {
  fn send_email(&self, address: &str, contents: &str) {}
}
