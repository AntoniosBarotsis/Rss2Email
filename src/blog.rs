use chrono::{DateTime, Utc};

/// Internal representation of a web feed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Blog {
  pub title: String,
  pub last_build_date: DateTime<Utc>,
  pub posts: Vec<Post>,
}

/// Internal representation of a web feed post.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Post {
  pub title: String,
  pub link: String,
  pub description: Option<String>,
  pub last_build_date: DateTime<Utc>,
}
