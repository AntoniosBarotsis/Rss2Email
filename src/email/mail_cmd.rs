//! Implementation for default `mail` command in linux.

use super::{email_provider::EmailProvider, EmailError};

#[derive(Default, Debug, Clone, Copy)]
pub struct MailCommand {}

impl EmailProvider for MailCommand {
  fn send_email(&self, address: &str, contents: &str) -> Result<(), EmailError> {
    send_email(address, contents)
  }
}

#[cfg(not(target_os = "windows"))]
fn send_email(address: &str, contents: &str) -> Result<(), EmailError> {
  use crate::info;
  use std::{fs::File, io::Write, process::Command};

  const TEMPORARY_FILE_NAME: &str = "/tmp/rss2-email.txt";

  let mut file = File::create(TEMPORARY_FILE_NAME).expect("Can't create temporary file");
  file
    .write_all(contents.as_bytes())
    .map_err(|_e| EmailError::Io("Failed to write temporary email file".to_owned()))?;

  let mail_command = format!("mail -s \"Rss2Email\" \"{address}\" < {TEMPORARY_FILE_NAME}");

  let mut mail_sender = Command::new("sh")
    .args(["-c", &mail_command])
    .spawn()
    .map_err(|_e| {
      EmailError::Other("Could not start mail command, is it installed and configured?".to_owned())
    })?;

  let exit_status = mail_sender.wait().expect("Mail command failed");
  info!("Mail command finished with status {exit_status}");

  match std::fs::remove_file(TEMPORARY_FILE_NAME) {
    Ok(_) => Ok(()),
    Err(e) => Err(EmailError::Io(format!(
      "Unable to delete {TEMPORARY_FILE_NAME} for error: {e}"
    ))),
  }
}

#[cfg(target_os = "windows")]
fn send_email(_address: &str, _contents: &str) -> Result<(), EmailError> {
  Err(EmailError::Config(
    "No known mail/sendmail/smtp command for Windows OS".to_owned(),
  ))
}
