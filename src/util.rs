use std::{fs, time::SystemTime};

use chrono::{DateTime, FixedOffset, Utc};
use log::{info, warn};
use std::fmt::Write as _;

use crate::{
  blog::{Blog, Post},
  xml::parse_xml,
};

/// Downloads all the RSS feeds specified in `feeds.txt` and converts them to `Blog`s.
pub(crate) fn download_blogs(days: i64) -> Vec<Blog> {
  let links = fs::read_to_string("feeds.txt").expect("Error in reading the feeds.txt file");

  let links = links.split('\n').map(|s| s.to_string());

  let contents: Vec<Blog> = links
    .into_iter()
    .filter(|link| !link.is_empty())
    .filter_map(|link| {
      let xml = get_page(&link);

      let xml = xml.unwrap();

      let res = parse_xml(xml);

      if res.is_err() {
        warn!("Error in {}\n{}", link, res.unwrap_err());
        return None;
      }

      res.ok()
    })
    .filter(|x| within_n_days(days, x.last_build_date))
    .map(|x| {
      let title = x.title;
      let last_build_date = x.last_build_date;
      let posts: Vec<Post> = x
        .posts
        .into_iter()
        .filter(|x| within_n_days(days, x.last_build_date))
        .collect();

      Blog {
        title,
        last_build_date,
        posts,
      }
    })
    .collect();

  contents
}

/// Generates the HTML contents corresponding to the given Blog collection.
pub(crate) fn map_to_html(blogs: &Vec<Blog>) -> String {
  let mut res = format!("<h1>Rss2Email - {}</h1>", Utc::now().date());

  for blog in blogs {
    let mut tmp = format!("<h2>{}</h2><ul>", blog.title);
    for post in &blog.posts {
      let _ = write!(tmp, "<li><a href=\"{}\">{}</a></li>", post.link, post.title);
    }
    tmp.push_str("</ul>");
    res.push_str(&tmp);
  }

  res
}

/// Returns true if the passed date is within `n` days from the current date.
fn within_n_days(n: i64, date: DateTime<FixedOffset>) -> bool {
  let today = Utc::now();

  let tz = date.timezone();
  let today = today.with_timezone(&tz);
  (today - date).num_days() <= n
}

/// Helper function for downloading the contents of a web page.
fn get_page(url: &str) -> Result<String, ureq::Error> {
  let body: String = ureq::get(url)
    .set("Example-Header", "header value")
    .call()?
    .into_string()?;

  Ok(body)
}

/// Helper function that times and prints the elapsed execution time
/// of `F` if ran in debug mode.
pub(crate) fn time_func<F, O>(f: F, fname: String) -> O
where
  F: Fn() -> O,
  O: Clone,
{
  let start = SystemTime::now();

  let res = f();

  let since_the_epoch = SystemTime::now()
    .duration_since(start)
    .expect("Time went backwards");

  if cfg!(debug_assertions) {
    info!(
      "Elapsed time for {} was {:?}ms",
      fname,
      since_the_epoch.as_millis()
    )
  }

  res
}