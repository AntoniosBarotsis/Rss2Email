use std::fmt::Display;

use quick_xml::DeError;

/// Represents possible issues that may arise when trying to parse web feeds.
/// If this occurs then a web feed is considered invalid.
#[derive(Debug, Eq, PartialEq, Hash)]
pub enum ParserError {
  /// Generic parsing error.
  Parse(String),
  /// Date format error.
  Date(DateError),
}

/// Represents different types of Date errors.
#[derive(Debug, Eq, PartialEq, Hash)]
pub enum DateError {
  Generic(String),
  TimeZoneError(String),
  Empty,
}

impl ParserError {
  /// Less verbose way of instantiating a [`DateError::Generic`].
  pub const fn generic_date_error(msg: String) -> Self {
    Self::Date(DateError::Generic(msg))
  }

  /// Less verbose way of instantiating a [`DateError::TimeZoneError`].
  pub const fn timezone_date_error(msg: String) -> Self {
    Self::Date(DateError::TimeZoneError(msg))
  }

  /// Less verbose way of instantiating a [`DateError::Empty`].
  pub const fn empty_date_error() -> Self {
    Self::Date(DateError::Empty)
  }
}

impl From<DeError> for ParserError {
  fn from(e: DeError) -> Self {
    Self::Parse(e.to_string())
  }
}

impl Display for ParserError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Parse(e) => write!(f, "Parse error: {e}"),
      Self::Date(e) => write!(f, "{e}"),
    }
  }
}

impl Display for DateError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Generic(e) => write!(f, "Date error: {e}"),
      Self::TimeZoneError(e) => write!(f, "Timezone error: {e}"),
      Self::Empty => write!(f, "Date was empty"),
    }
  }
}
