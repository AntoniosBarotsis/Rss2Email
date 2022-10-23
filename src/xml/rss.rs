use chrono::{DateTime, Utc};
//! [Specification](https://www.rssboard.org/rss-specification)
//!
//! ```xml
//! <rss>
//!   <channel>
//!     <title></title>
//!     <lastBuildDate>RFC 2822</lastBuildDate>
//!     <pubDate>RFC 2822</pubDate>
//!     <item>
//!       <title></title>
//!       <link></link>
//!       <pubDate>RFC 2822</pubDate>
//!       <description></description>?
//!     </item>
//!   </channel>
//! </rss>
//! ```
use log::warn;
use quick_xml::DeError;
use serde_derive::{Deserialize, Serialize};

use crate::blog::{Blog, Post};

use super::{
  traits::{BlogPost, WebFeed},
  ParserError,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename = "rss")]
pub struct RssFeed {
  pub channel: Channel,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
  pub title: String,
  pub last_build_date: Option<String>,
  pub pub_date: Option<String>,
  #[serde(rename = "item")]
  pub items: Option<Vec<RssPost>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "item")]
pub struct RssPost {
  // Link and title can be omitted, according to spec, provided that there is a description
  // https://www.rssboard.org/rss-specification#hrelementsOfLtitemgt
  pub title: Option<String>,
  pub link: Option<String>,
  pub description: Option<String>,
  pub pub_date: Option<String>,
}

impl WebFeed for Result<RssFeed, DeError> {
  fn into_blog(self) -> Result<Blog, ParserError> {
    let feed = self?;

    let title = feed.channel.title;

    let site_last_build_date = feed.channel.pub_date;
    let items = feed.channel.items.unwrap_or_default();
    let last_post_build_date = items.first().and_then(|x| x.clone().pub_date);

    let last_build_date = site_last_build_date
      .or(last_post_build_date)
      .ok_or_else(|| ParserError::Parse("Date not found.".to_owned()))?;

    let posts: Vec<Post> = items
      .iter()
      .filter_map(|x| match x.clone().into_post() {
        Ok(post) => Some(post),
        Err(e) => {
          warn!(
            "\"{}\"'s post titled \"{}\" errored with {}",
            title,
            x.title
              .as_ref()
              .map_or_else(|| "n/a".to_string(), std::clone::Clone::clone),
            e
          );
          None
        }
      })
      .collect();

    match DateTime::parse_from_rfc2822(&last_build_date) {
      Ok(last_build_date) => Ok(Blog {
        title,
        last_build_date: last_build_date.with_timezone(&Utc),
        posts,
      }),
      Err(e) => Err(ParserError::Parse(e.to_string())),
    }
  }
}

impl BlogPost for RssPost {
  fn into_post(self) -> Result<Post, ParserError> {
    let link = if let Some(link) = self.link {
      link
    } else {
      return Err(ParserError::Parse("No link in post".to_string()));
    };

    let (title, description) = match (self.title, self.description) {
      (Some(link), description) => (link, description),
      (None, None) => (link.clone(), None),
      (None, Some(description)) => {
        if description.len() > 50 {
          (format!("{}...", &description[0..50]), Some(description))
        } else {
          (description, None)
        }
      }
    };

    let pub_date = self
      .pub_date
      .ok_or_else(|| ParserError::Parse("Date not found.".to_owned()))?;

    match DateTime::parse_from_rfc2822(&pub_date) {
      Ok(last_build_date) => Ok(Post {
        title,
        link,
        description,
        last_build_date: last_build_date.with_timezone(&Utc),
      }),
      Err(e) => Err(ParserError::Date(e.to_string())),
    }
  }
}
