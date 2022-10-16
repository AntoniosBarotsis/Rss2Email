/**
 * Implementation for default "mail" command in linux
 */
use super::email_provider::EmailProvider;

#[derive(Default)]
pub struct MailCommand {}

impl EmailProvider for MailCommand {
  fn send_email(&self, address: &str, contents: &str) {
    send_email(address, contents);
  }
}

#[cfg(not(target_os = "windows"))]
fn send_email(address: &str, contents: &str) {
  use log::info;
  use std::{fs::File, io::Write, process::Command};

  let mut file =
    File::create("/tmp/rss2-email.txt").expect("Can't create temporary file /tmp/rss2-email.txt");
  file
    .write_all(contents.as_bytes())
    .expect("Failed to write temporary email");

  let mail_command = format!("sendmail -s \"Rss2Email\" \"{address}\" < /tmp/rss2-email.txt");

  let mut mail_sender = Command::new("sh")
    .args(["-c", &mail_command])
    .spawn()
    .expect("Could not start mail command, is it installed and configured?");

  let exit_status = mail_sender.wait().expect("Mail command failed");
  info!("Mail command finished with status {exit_status}");
}

#[cfg(target_os = "windows")]
fn send_email(address: &str, contents: &str) {
  use log::error;
  error!("No known mail/sendmail/smtp command for Windows OS");
}
