use itertools::Itertools;
use serde_xml_rs::from_str;

use crate::blog::Blog;

use self::{atom::Feed, rss::Rss, traits::ResultToBlog};

pub mod atom;
pub mod rss;
mod traits;

/// Turns an XML feed into a `Blog` if possible.
pub fn parse_web_feed(xml: &str) -> Result<Blog, String> {
  let possible_roots = vec![
    from_str::<Rss>(xml).into_blog(),
    from_str::<Feed>(xml).into_blog(),
  ];

  let (roots, errors): (Vec<_>, Vec<_>) = possible_roots.into_iter().partition_result();

  roots
    .first()
    .cloned()
    .ok_or_else(|| format!("{:?}", errors.iter().unique().collect::<Vec<_>>()))
}
