use log::{error, info};

/// Sends an email to and from the specified address.
pub fn send_email(address: &str, api_key: &str, contents: &str) {
  let contents = contents.replace('\"', "\\\"");
  let message = format!(
    r#"{{"personalizations": [{{"to": [{{"email": "{address}"}}]}}],"from": {{"email": "{address}"}},"subject": "Rss2Email","content": [{{"type": "text/html", "value": "{contents}"}}]}}"#
  );

  let req = ureq::post("https://api.sendgrid.com/v3/mail/send")
    .set("Authorization", &format!("Bearer {}", &api_key))
    .set("Content-Type", "application/json")
    .send_string(&message);

  match req {
    Ok(req) => info!(
      "Email request sent with {} {}",
      req.status(),
      req.status_text()
    ),
    Err(e) => error!("{}", e),
  }
}
