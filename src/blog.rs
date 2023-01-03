use chrono::{DateTime, Utc};

/// Internal representation of a web feed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Blog {
  pub title: String,
  pub most_recent_pub_date: DateTime<Utc>,
  pub posts: Vec<Post>,
}

/// Internal representation of a web feed post.
///
/// The `pub_date` field will prefer the publication date
/// and fallback to the last update date.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Post {
  pub title: String,
  pub link: String,
  pub description: Option<String>,
  pub pub_date: DateTime<Utc>,
}
