/// [Specification](https://www.rssboard.org/rss-specification)
///
/// <rss>
///   <channel>
///     <title></title>
///     <lastBuildDate>RFC 2822</lastBuildDate>
///     <pubDate>RFC 2822</pubDate>
///     <item>
///       <title></title>
///       <link></link>
///       <pubDate>RFC 2822</pubDate>
///       <description></description>?
///     </item>
///   </channel>
/// </rss>
use chrono::DateTime;
use log::warn;
use serde_derive::{Deserialize, Serialize};
use serde_xml_rs::Error;

use crate::blog::{Blog, Post};

use super::traits::{BlogPost, ResultToBlog, WebFeed};

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
  pub items: Vec<RssPost>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "item")]
pub struct RssPost {
  pub title: String,
  pub link: String,
  pub description: Option<String>,
  pub pub_date: Option<String>,
}

impl WebFeed for RssFeed {
  fn into_blog(self) -> Result<Blog, String> {
    let title = self.channel.title;

    let site_last_build_date = self.channel.pub_date;
    let last_post_build_date = self.channel.items.first().and_then(|x| x.clone().pub_date);

    let last_build_date = site_last_build_date
      .or(last_post_build_date)
      .ok_or_else(|| "Date not found.".to_owned())?;

    let posts: Vec<Post> = self
      .channel
      .items
      .iter()
      .filter_map(|x| match x.clone().into_post() {
        Ok(post) => Some(post),
        Err(e) => {
          warn!(
            "\"{}\"'s post titled \"{}\" errored with {}",
            title, x.title, e
          );
          None
        }
      })
      .collect();

    match DateTime::parse_from_rfc2822(&last_build_date) {
      Ok(last_build_date) => Ok(Blog {
        title,
        last_build_date,
        posts,
      }),
      Err(e) => Err(format!("Date error: {}", e)),
    }
  }
}

impl BlogPost for RssPost {
  fn into_post(self) -> Result<Post, String> {
    let title = self.title;
    let link = self.link;
    let description = self.description;

    let pub_date = self.pub_date.ok_or_else(|| "Date not found.".to_owned())?;

    match DateTime::parse_from_rfc2822(&pub_date) {
      Ok(last_build_date) => Ok(Post {
        title,
        link,
        description,
        last_build_date,
      }),
      Err(e) => Err(format!("Date error: {}", e)),
    }
  }
}

impl ResultToBlog<RssFeed> for Result<RssFeed, Error> {
  fn into_blog(self) -> Result<Blog, String> {
    match self {
      Ok(res) => res.into_blog(),
      Err(e) => Err(e.to_string()),
    }
  }
}
