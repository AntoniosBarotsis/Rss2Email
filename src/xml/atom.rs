/// [Specification](https://www.rfc-editor.org/rfc/rfc4287)
///
/// <feed>
///   <title></title>
///   <updated>ISO.8601</updated>
///   <entry>
///     <title></title>
///     <link href=""/>
///     <updated>ISO.8601</updated>
///     <summary></summary>?
///   </entry>
/// </feed>
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use serde_xml_rs::Error;

use crate::blog::{Blog, Post};

use super::traits::{BlogPost, ResultToBlog, WebFeed};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "feed")]
pub struct AtomFeed {
  pub title: String,
  #[serde(rename = "entry")]
  pub entries: Vec<AtomPost>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "entry")]
pub struct AtomPost {
  pub title: String,
  pub link: Link,
  pub summary: Option<String>,
  pub updated: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Link {
  href: String,
}

impl WebFeed for AtomFeed {
  fn into_blog(self) -> Result<Blog, String> {
    let title = self.title;
    let posts: Vec<Post> = self
      .entries
      .iter()
      .filter_map(|x| x.clone().into_post().ok())
      .collect();

    let last_build_date = posts
      .iter()
      .map(|x| x.last_build_date)
      .max()
      .ok_or("Date error")?;

    Ok(Blog {
      title,
      last_build_date,
      posts,
    })
  }
}

impl BlogPost for AtomPost {
  fn into_post(self) -> Result<Post, String> {
    let title = self.title;
    let link = self.link.href;
    let description = self.summary;
    let pub_date = self.updated;

    match DateTime::parse_from_rfc3339(&pub_date) {
      Ok(last_build_date) => Ok(Post {
        title,
        link,
        description,
        last_build_date: last_build_date.with_timezone(&Utc),
      }),
      Err(e) => Err(format!("Date error: {}", e)),
    }
  }
}

impl ResultToBlog<AtomFeed> for Result<AtomFeed, Error> {
  fn into_blog(self) -> Result<Blog, String> {
    match self {
      Ok(res) => res.into_blog(),
      Err(e) => Err(e.to_string()),
    }
  }
}
