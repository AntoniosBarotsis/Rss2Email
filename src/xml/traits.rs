use crate::blog::{Blog, Post};

/// Represents a web feed that can be converted to a `blog.Blog`.
pub trait WebFeed {
  fn into_blog(self) -> Result<Blog, String>;
}

/// Represents an object that can be converted to a `blog.Post`.
pub trait BlogPost {
  fn into_post(self) -> Result<Post, String>;
}

/// Helper wrapper for `Result<T, String>` where `T: XmlFeed`,
pub trait ResultToBlog<T>
where
  T: WebFeed,
{
  fn into_blog(self) -> Result<Blog, String>;
}
