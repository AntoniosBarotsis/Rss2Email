//! [Specification](https://www.rfc-editor.org/rfc/rfc4287)
//!
//! ```xml
//! <feed>
//!   <title></title>
//!   <updated>ISO.8601</updated>
//!   <entry>
//!     <title></title>
//!     <link href=""/>
//!     <updated>ISO.8601</updated>
//!     <published>ISO.8601</published>?
//!     <summary></summary>?
//!   </entry>
//! </feed>
//! ```

use chrono::{DateTime, Utc};
use log::warn;
use quick_xml::DeError;
use serde_derive::{Deserialize, Serialize};

use crate::blog::{Blog, Post};

use super::{
  traits::{BlogPost, WebFeed},
  ParserError,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "feed")]
pub struct AtomFeed {
  pub title: String,
  #[serde(rename = "entry", default)]
  pub entries: Vec<AtomPost>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "entry")]
pub struct AtomPost {
  pub title: String,
  #[serde(rename = "link")]
  pub links: Vec<Link>,
  pub summary: Option<String>,
  pub published: Option<String>,
  pub updated: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Link {
  // See https://github.com/tafia/quick-xml/issues/534
  #[serde(rename = "@href")]
  href: String,
}

impl WebFeed for Result<AtomFeed, DeError> {
  fn into_blog(self) -> Result<Blog, ParserError> {
    let feed = self?;
    let title = feed.title;

    let posts: Vec<Post> = feed
      .entries
      .iter()
      // TODO Turn this into a method
      .filter_map(|x| match x.clone().into_post() {
        Ok(post) => Some(post),
        Err(e) => {
          warn!(
            "\"{}\"'s post titled \"{}\" errored with '{}'",
            title, x.title, e
          );
          None
        }
      })
      .collect::<Vec<_>>();

    if posts.is_empty() {
      return Err(ParserError::Parse(format!("Empty feed: {title}")));
    }

    let last_build_date = posts
      .iter()
      .map(|x| x.last_build_date)
      .max()
      .ok_or_else(|| ParserError::Parse("Date error.".to_owned()))?;

    Ok(Blog {
      title,
      last_build_date,
      posts,
    })
  }
}

impl BlogPost for AtomPost {
  fn into_post(self) -> Result<Post, ParserError> {
    let title = self.title;
    // Use the first link for now
    let link = self.links[0].href.clone();
    let description = self.summary;
    // Use publish date if exists otherwise fallback to updated
    let pub_date = self.published.unwrap_or(self.updated);

    if pub_date.is_empty() {
      return Err(ParserError::empty_date_error());
    }

    match DateTime::parse_from_rfc3339(&pub_date) {
      Ok(last_build_date) => Ok(Post {
        title,
        link,
        description,
        last_build_date: last_build_date.with_timezone(&Utc),
      }),
      Err(e) => Err(ParserError::generic_date_error(format!(
        "Error parsing date '{}' ({})",
        pub_date, e
      ))),
    }
  }
}
