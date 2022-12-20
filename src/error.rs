/// Represents anything that could go wrong when dealing with web feeds.
#[derive(Debug)]
pub enum Error {
  /// Wrapper for [reqwest::Error].
  Reqwest(Box<reqwest::Error>),
  /// Wrapper for [http::header::ToStrError].
  HeaderString(Box<http::header::ToStrError>),
  /// Wrapper for [std::io::Error].
  Io(std::io::Error),
  /// 
  Generic(String),
}