/// Sends an email to and from the specified address.
pub(crate) fn send_email(address: String, api_key: String, contents: String) {
  let contents = contents.replace('\"', "\\\"");
  let message = format!(
    r#"{{"personalizations": [{{"to": [{{"email": "{address}"}}]}}],"from": {{"email": "{address}"}},"subject": "Rss2Email","content": [{{"type": "text/html", "value": "{contents}"}}]}}"#
  );

  let req = ureq::post("https://api.sendgrid.com/v3/mail/send")
    .set("Authorization", &format!("Bearer {}", &api_key))
    .set("Content-Type", "application/json")
    .send_string(&message);

  println!("{:?}", req)
}
