//! Parses web feeds according to the RSS and Atom specifications and constructs
//! [`Blog`]s and [`Post`](crate::blog::Post)s.

use std::fmt::Display;

use quick_xml::{de::from_str, DeError};

use crate::blog::Blog;

use self::{atom::AtomFeed, rss::RssFeed, traits::WebFeed};

pub mod atom;
pub mod rss;
mod traits;

/// Represents possible issues that may arise when trying to parse web feeds.
/// If this occurs then a web feed is considered invalid.
#[derive(Debug, Eq, PartialEq, Hash)]
pub enum ParserError {
  /// Generic parsing error.
  Parse(String),
  /// Date format error.
  Date(String),
}

/// Turns an XML feed into a `Blog` if possible.
///
/// First tries to parse it into an [`RssFeed`]. If that fails,
/// it then tries to parse it into an [`AtomFeed`]. If both fail,
/// the error is set to `Error1. Error2`.
pub fn parse_web_feed(xml: &str) -> Result<Blog, ParserError> {
  from_str::<RssFeed>(xml).into_blog().or_else(|e1| {
    from_str::<AtomFeed>(xml)
      .into_blog()
      .map_err(|e2| ParserError::Parse(format!("{} {}", e1, e2)))
  })
}

impl From<DeError> for ParserError {
  fn from(e: DeError) -> Self {
    Self::Parse(e.to_string())
  }
}

impl Display for ParserError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Parse(e) => write!(f, "Parse error: {}", e),
      Self::Date(e) => write!(f, "Date error: {}", e),
    }
  }
}
