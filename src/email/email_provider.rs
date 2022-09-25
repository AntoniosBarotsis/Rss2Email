/// An email provider abstraction to allow for multiple backends.
pub trait EmailProvider {
  /// Sends an email to and from the specified address.
  fn send_email(&self, address: &str, api_key: &str, contents: &str);
}
