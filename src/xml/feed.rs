use chrono::DateTime;
use serde_derive::{Deserialize, Serialize};
use serde_xml_rs::Error;

use crate::blog::{Blog, Post};

use super::traits::{BlogPost, ResultToBlog, XmlFeed};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Feed {
  pub title: String,
  pub updated: Option<String>,
  #[serde(rename = "entry")]
  pub entries: Vec<Entry>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
  pub title: String,
  pub link: String,
  pub description: Option<String>,
  pub summary: Option<String>,
  pub pub_date: Option<String>,
}

impl XmlFeed for Feed {
  fn into_blog(self) -> Result<Blog, String> {
    let title = self.title;
    let last_build_date = self.updated;
    let posts: Vec<Post> = self
      .entries
      .iter()
      .filter_map(|x| x.clone().into_post().ok())
      .collect();

    if last_build_date.is_none() {
      return Err("Date not found.".to_owned());
    }

    match DateTime::parse_from_rfc3339(&last_build_date.unwrap()) {
      Ok(last_build_date) => Ok(Blog {
        title,
        last_build_date,
        posts,
      }),
      Err(e) => Err(format!("Date error: {}", e)),
    }
  }
}

impl BlogPost for Entry {
  fn into_post(self) -> Result<Post, String> {
    let title = self.title;
    let link = self.link;
    let description = self
      .description
      .or(self.summary)
      .unwrap_or_else(|| "".to_owned());

    if self.pub_date.is_none() {
      return Err("Date not found.".to_owned());
    }

    match DateTime::parse_from_rfc2822(&self.pub_date.unwrap()) {
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

impl ResultToBlog<Feed> for Result<Feed, Error> {
  fn into_blog(self) -> Result<Blog, String> {
    match self {
      Ok(res) => res.into_blog(),
      Err(e) => Err(e.to_string()),
    }
  }
}
