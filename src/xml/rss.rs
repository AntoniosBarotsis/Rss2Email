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

use chrono::{DateTime, FixedOffset, TimeZone, Utc};
use log::warn;
use quick_xml::DeError;
use regex::Regex;
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

    let last_build_date = parse_date_helper(&last_build_date)?;

    Ok(Blog {
      title,
      last_build_date: last_build_date.with_timezone(&Utc),
      posts,
    })
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

    let last_build_date = parse_date_helper(&pub_date)?;

    Ok(Post {
      title,
      link,
      description,
      last_build_date: last_build_date.with_timezone(&Utc),
    })
  }
}

/// Helper method that first tries to parse a date using [`DateTime::parse_from_rfc2822`]
/// and if that fails, it tries with [`parse_from_rfc822`].
fn parse_date_helper(date: &str) -> Result<DateTime<FixedOffset>, ParserError> {
  DateTime::parse_from_rfc2822(date).or_else(|_| parse_from_rfc822(date))
}

/// Tries to parse [`RFC822`](https://www.w3.org/Protocols/rfc822/#z28). This is a much not
/// *complete* solution since very few timezones are currently supported (see [`tz_to_offset`])
/// but it works for now and it is not used frequently. I will be updating it whenever I find
/// feeds that break it.
fn parse_from_rfc822(date: &str) -> Result<DateTime<FixedOffset>, ParserError> {
  let format_str = "%d %b %y %H:%M";
  let regex = Regex::new(r"\s?([a-zA-Z]+$)").expect("Invalid regex");

  let cap = regex
    .captures(date)
    .and_then(|x| x.get(1))
    .ok_or_else(|| ParserError::Date("Timezone not found".to_string()))?
    .as_str();

  let date = regex.replace_all(date, "").to_string();

  let tz = tz_to_offset(cap)?;

  tz.datetime_from_str(&date, format_str)
    .map_err(|_e| ParserError::Date(format!("Date \"{}\" could not be parsed.", date)))
}

/// Maps timezones from Strings to [`FixedOffset`]s
fn tz_to_offset(tz: &str) -> Result<FixedOffset, ParserError> {
  match tz {
    "UTC" => Ok(FixedOffset::east(0)),
    _ => Err(ParserError::Date(format!(
      "Unknown timezone {}, please open an issue!",
      tz
    ))),
  }
}
