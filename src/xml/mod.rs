use itertools::Itertools;
use serde_xml_rs::from_str;

use crate::blog::Blog;

use self::{feed::Feed, rss::Rss, traits::ResultToBlog};

pub mod feed;
pub mod rss;
mod traits;

/// Turns XML text into a `Blog` if possible.
pub fn parse_xml(xml: &str) -> Result<Blog, String> {
  let possible_roots = vec![
    from_str::<Rss>(xml).into_blog(),
    from_str::<Feed>(xml).into_blog(),
  ];

  let actual_root = possible_roots.iter().find_map(|x| x.as_ref().ok());

  match actual_root {
    Some(res) => Ok(res.to_owned()),
    None => {
      let errs: Vec<&String> = possible_roots
        .iter()
        .map(|x| x.as_ref().unwrap_err())
        .unique()
        .collect();

      Err(format!("{:?}", errs))
    }
  }
}
