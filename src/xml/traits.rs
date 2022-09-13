use crate::blog::{Blog, Post};

/// Represents an object that can be converted to a `blog.Post`.
pub trait BlogPost {
  fn into_post(self) -> Result<Post, String>;
}

/// Represents an XML feed that can be converted to a `blog.Blog`.
pub trait XmlFeed {
  fn into_blog(self) -> Result<Blog, String>;
}

/// Helper wrapper for `Result<T, String>` where `T: XmlFeed`,
pub trait ResultToBlog<T>
where
  T: XmlFeed,
{
  fn into_blog(self) -> Result<Blog, String>;
}
