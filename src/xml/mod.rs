//! Parses web feeds according to the RSS and Atom specifications and constructs
//! [`Blog`]s and [`Post`](crate::blog::Post)s.

use quick_xml::de::from_str;

use crate::blog::Blog;

use self::{atom::AtomFeed, error::ParserError, rss::RssFeed, traits::WebFeed};

pub mod atom;
pub mod error;
pub mod rss;
mod traits;

/// Turns an XML feed into a `Blog` if possible.
///
/// First tries to parse it into an [`RssFeed`]. If that fails,
/// it then tries to parse it into an [`AtomFeed`]. If both fail,
/// the error is set to `Error1. Error2`.
pub fn parse_web_feed(xml: &str) -> Result<Blog, ParserError> {
  from_str::<RssFeed>(xml).into_blog().or_else(|e1| {
    from_str::<AtomFeed>(xml)
      .into_blog()
      .map_err(|e2| ParserError::Parse(format!("{e1}\n{e2}")))
  })
}
