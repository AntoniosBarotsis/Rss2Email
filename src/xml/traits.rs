use crate::blog::{Blog, Post};

use super::ParserError;

/// Represents a web feed that can be converted to a `blog.Blog`.
pub trait WebFeed {
  fn into_blog(self) -> Result<Blog, ParserError>;
}

/// Represents an object that can be converted to a `blog.Post`.
pub trait BlogPost {
  fn into_post(self) -> Result<Post, ParserError>;
}
