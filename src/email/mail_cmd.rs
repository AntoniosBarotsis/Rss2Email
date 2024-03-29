//! Implementation for default `mail` command in linux.

use super::{email_provider::EmailProvider, error::EmailError};

#[derive(Default, Debug, Clone, Copy)]
pub struct MailCommand {}

impl EmailProvider for MailCommand {
  fn send_email(
    &self,
    from_address: &str,
    recipient_addresses: Vec<&str>,
    subject: &str,
    contents: &str,
  ) -> Result<(), EmailError> {
    send_email(from_address, &recipient_addresses, subject, contents)
  }
}

#[cfg(not(target_os = "windows"))]
fn send_email(
  from_address: &str,
  recipient_addresses: &[&str],
  subject: &str,
  contents: &str,
) -> Result<(), EmailError> {
  use crate::info;
  use std::{fs::File, io::Write, process::Command};

  const TEMPORARY_FILE_NAME: &str = "/tmp/rss2-email.txt";

  let mut file = File::create(TEMPORARY_FILE_NAME).expect("Can't create temporary file");
  file
    .write_all(contents.as_bytes())
    .map_err(|_e| EmailError::Io("Failed to write temporary email file".to_owned()))?;

  let recipients = recipient_addresses.join(",");
  let mail_command =
    format!("mail -s \"{subject}\" \"{recipients}\" -aFrom:{from_address} < {TEMPORARY_FILE_NAME}");

  let mut mail_sender = Command::new("sh")
    .args(["-c", &mail_command])
    .spawn()
    .map_err(|_e| {
      EmailError::Other("Could not start mail command, is it installed and configured?".to_owned())
    })?;

  let exit_status = mail_sender.wait().expect("Mail command failed");
  info!("Mail command finished with status {exit_status}");

  match std::fs::remove_file(TEMPORARY_FILE_NAME) {
    Ok(()) => Ok(()),
    Err(e) => Err(EmailError::Io(format!(
      "Unable to delete {TEMPORARY_FILE_NAME} for error: {e}"
    ))),
  }
}

#[cfg(target_os = "windows")]
fn send_email(
  _from_address: &str,
  _recipient_addresses: &[&str],
  _subject: &str,
  _contents: &str,
) -> Result<(), EmailError> {
  Err(EmailError::Config(
    "No known mail/sendmail/smtp command for Windows OS".to_owned(),
  ))
}
