use std::fmt::Display;

use itertools::Itertools;
use quick_xml::{de::from_str, DeError};

use crate::blog::Blog;

use self::{atom::AtomFeed, rss::RssFeed, traits::WebFeed};

pub mod atom;
pub mod rss;
mod traits;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum ParserError {
  Parse(String),
  Date(String),
}

/// Turns an XML feed into a `Blog` if possible.
pub fn parse_web_feed(xml: &str) -> Result<Blog, ParserError> {
  let possible_roots = vec![
    from_str::<RssFeed>(xml).into_blog(),
    from_str::<AtomFeed>(xml).into_blog(),
  ];

  let (roots, errors): (Vec<_>, Vec<_>) = possible_roots.into_iter().partition_result();

  roots
    .first()
    .cloned()
    .ok_or_else(|| ParserError::Parse(format!("{:?}", errors.iter().unique().collect::<Vec<_>>())))
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
