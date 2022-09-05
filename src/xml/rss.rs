use chrono::DateTime;
use serde::{Deserialize, Serialize};
use serde_xml_rs::Error;

use crate::blog::{Blog, Post};

use super::traits::{BlogPost, ResultToBlog, XmlFeed};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Rss {
  pub channel: Channel,
}

impl XmlFeed for Rss {
  fn into_blog(self) -> Result<Blog, String> {
    let title = self.channel.title;

    // let last_build_date =
    //   self
    //     .channel
    //     .last_build_date
    //     .or(self.channel.pub_date)
    //     .or(match self.channel.items.first() {
    //       Some(item) => item.to_owned().pub_date,
    //       None => None,
    //     });

    let last_build_date =
      self
        .channel.pub_date
        .or(match self.channel.items.first() {
          Some(item) => item.to_owned().pub_date,
          None => None,
        });

    let posts: Vec<Post> = self
      .channel
      .items
      .iter()
      .filter_map(|x| x.clone().into_post().ok())
      .collect();

    if last_build_date.is_none() {
      return Err("Date not found.".to_owned());
    }

    match DateTime::parse_from_rfc2822(&last_build_date.unwrap()) {
      Ok(last_build_date) => Ok(Blog {
        title,
        last_build_date,
        posts,
      }),
      Err(e) => Err(format!("Date error: {}", e)),
    }
  }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
  pub title: String,
  // https://github.com/RReverser/serde-xml-rs/issues/64#issuecomment-1231886555
  // #[serde(rename = "link", default)]
  // pub link: String,
  pub last_build_date: Option<String>,
  pub pub_date: Option<String>,
  #[serde(rename = "item")]
  pub items: Vec<Item>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Item {
  pub title: String,
  pub link: String,
  pub description: String,
  pub pub_date: Option<String>,
}

impl BlogPost for Item {
  fn into_post(self) -> Result<Post, String> {
    let title = self.title;
    let link = self.link;
    let description = self.description;

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

impl ResultToBlog<Rss> for Result<Rss, Error> {
  fn into_blog(self) -> Result<Blog, String> {
    match self {
      Ok(res) => res.into_blog(),
      Err(e) => Err(e.to_string()),
    }
  }
}
