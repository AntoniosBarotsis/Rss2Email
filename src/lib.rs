use std::{fs, time::SystemTime};

use chrono::{DateTime, Utc};
use futures::{stream, StreamExt};
use itertools::Itertools;
use lazy_static::lazy_static;
use log::{info, warn};
use regex::Regex;
use reqwest::Client;
use std::fmt::Write as _;
use tokio::runtime::Handle;

pub use blog::{Blog, Post};
mod blog;
mod xml;

use crate::xml::parse_web_feed;

const CONCURRENT_REQUESTS: usize = 10;

pub async fn get_blogs(links: Vec<String>) -> Vec<Option<Blog>> {
  let client = Client::new();
  stream::iter(links)
    .map(|link| {
      let client = &client;
      async move {
        let xml = get_page_async(link.as_str(), client)
          .await
          .map_err(|e| warn!("Error in {}\n{:?}", link, e))
          .ok()?;

        parse_web_feed(&xml)
          .map_err(|e| warn!("Error in {}\n{}", link, e))
          .ok()
      }
    })
    .buffer_unordered(CONCURRENT_REQUESTS)
    .collect::<Vec<Option<Blog>>>()
    .await
}

/// Downloads all the RSS feeds specified in `feeds.txt` and converts them to `Blog`s.
pub fn download_blogs(days: i64) -> Vec<Blog> {
  let links = read_feeds();

  let contents = match Handle::try_current() {
    Ok(handle) => std::thread::spawn( move || handle.block_on(get_blogs(links))).join().expect("Error spawning blog download"),
    Err(_) => {
      let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Could not build tokio runtime");

      rt.block_on(get_blogs(links))
    }
  };

  let contents: Vec<Blog> = contents
    .into_iter()
    .filter_map(|x| match x {
      Some(x) => {
        if !within_n_days(days, &x.last_build_date) {
          return None;
        }

        let recent_posts: Vec<Post> = x
          .posts
          .into_iter()
          .filter(|x| within_n_days(days, &x.last_build_date))
          .collect();

        let non_empty = !recent_posts.is_empty();

        non_empty.then_some(Blog {
          posts: recent_posts,
          ..x
        })
      }
      None => None,
    })
    .collect();

  contents
}

/// Parses links from `feeds.txt`.
///
/// Assumed one link per line. Any text between a `#` and a line end
/// is considered a comment.
pub fn read_feeds() -> Vec<String> {
  let links = std::env::var("FEEDS")
    .or_else(|_| fs::read_to_string("feeds.txt"))
    .expect("Error in reading the feeds");

  // Not really necessary but yes
  // https://docs.rs/regex/latest/regex/#example-avoid-compiling-the-same-regex-in-a-loop
  lazy_static! {
    static ref RE: Regex = #[allow(clippy::unwrap_used)]
    Regex::new(r"#.*$").unwrap();
  }

  links
    .split(feeds_splitter)
    .map(std::string::ToString::to_string)
    .map(|l| RE.replace_all(&l, "").to_string())
    .map(|l| l.trim().to_owned())
    .filter(|l| !l.is_empty())
    .unique()
    .collect::<Vec<String>>()
}

/// Splits the feeds on either
///
/// - `\n` for input coming from `feeds.txt`
/// - `;`  for input coming from an environment variable
const fn feeds_splitter(c: char) -> bool {
  c == '\n' || c == ';'
}

/// Generates the HTML contents corresponding to the given Blog collection.
pub fn map_to_html(blogs: &Vec<Blog>) -> String {
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
fn within_n_days(n: i64, date: &DateTime<Utc>) -> bool {
  let today = Utc::now();
  let date = date.with_timezone(&Utc);
  (today - date).num_days() <= n
}

#[derive(Debug)]
pub enum DownloadError {
  Ureq(Box<ureq::Error>),
  Reqwest(Box<reqwest::Error>),
  Io(std::io::Error),
  Custom(String),
}

impl From<std::io::Error> for DownloadError {
  fn from(error: std::io::Error) -> Self {
    Self::Io(error)
  }
}

impl From<ureq::Error> for DownloadError {
  fn from(error: ureq::Error) -> Self {
    Self::Ureq(Box::new(error))
  }
}

impl From<reqwest::Error> for DownloadError {
  fn from(error: reqwest::Error) -> Self {
    Self::Reqwest(Box::new(error))
  }
}

fn is_supported_content(content_type: &str) -> bool {
  let supported = vec![
    "application/xml",
    "text/xml",
    "application/rss+xml",
    "application/atom+xml",
  ];
  supported.contains(&content_type)
}

/// Helper function for downloading the contents of a web page.
pub fn get_page(url: &str) -> Result<String, DownloadError> {
  let response = ureq::get(url).call()?;

  if !is_supported_content(response.content_type()) {
    return Err(DownloadError::Custom(format!(
      "Invalid content {} for {}",
      response.content_type(),
      url
    )));
  }

  let body = response.into_string()?;
  Ok(body)
}

/// Helper function for downloading the contents of a web page.
pub async fn get_page_async(url: &str, client: &Client) -> Result<String, DownloadError> {
  let response = client
    .get(url)
    .header(
      "Accept",
      "application/xml, text/xml, application/rss+xml, application/atom+xml",
    )
    .header("User-Agent", "Rss2Email");
  let response = response.send().await?;

  let content_type = response.headers().get(reqwest::header::CONTENT_TYPE);

  match content_type {
    Some(content_type) => {
      match content_type.to_str() {
        Ok(content_type) => {
          let content_type = content_type.split(';').collect::<Vec<&str>>()[0].to_owned();
          if !is_supported_content(content_type.as_str()) {
            return Err(DownloadError::Custom(format!(
              "Invalid content {} for {}",
              content_type.as_str(),
              url
            )));
          }
          let body = response.text().await;

          // let body = response.text().await;
          match body {
            Ok(body) => Ok(body),
            Err(_) => Err(DownloadError::Custom("Body decode error".to_string())),
          }
        }
        Err(_) => Err(DownloadError::Custom(
          "Content Type parsing error".to_string(),
        )),
      }
    }
    None => Err(DownloadError::Custom(
      "No content type header found on request.".to_string(),
    )),
  }
}
/// Helper function that times and prints the elapsed execution time
/// of `F` if ran in debug mode.
pub fn time_func<F, O>(f: F, fname: &str) -> O
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
    );
  }

  res
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_download_plain_xml() {
    let payload = get_page("https://antoniosbarotsis.github.io/index.xml");
    let content = payload.expect("Downloaded content");
    assert!(content.starts_with("<?xml"));
    assert!(content.contains("<rss"));
    assert!(content.ends_with("</rss>"));
  }

  #[test]
  fn test_download_xml_for_rss() {
    let payload = get_page("https://github.blog/feed");
    let content = payload.expect("Downloaded content");
    assert!(content.starts_with("<?xml"));
  }

  #[test]
  fn test_download_invalid_page() {
    let payload = get_page("https://antoniosbarotsis.github.io/ordex.pkxml");
    assert!(payload.is_err());
  }

  #[test]
  fn test_download_with_text() {
    // Text should not be confused with xml: here, we received html for a markdown
    let url = "https://github.com/AntoniosBarotsis/Rss2Email/raw/cc5b2bee846f9dab8f5787dfcb9a01d963321630/README.md";
    let payload = get_page(url);
    assert!(payload.is_err());
    let error = payload.unwrap_err();
    if let DownloadError::Custom(message) = error {
      assert!(message.contains("Invalid content"));
      assert!(message.contains(url));
    } else {
      panic!("Unexpected error {:?}", error);
    }
  }

  #[test]
  fn test_download_with_an_image() {
    // Using an URL to a specific sha of this github repo to make sure the target remains
    let url = "https://github.com/AntoniosBarotsis/Rss2Email/raw/cc5b2bee846f9dab8f5787dfcb9a01d963321630/assets/res.jpg";
    let payload = get_page(url);
    assert!(payload.is_err());
    let error = payload.unwrap_err();
    if let DownloadError::Custom(message) = error {
      assert!(message.contains("Invalid content"));
      assert!(message.contains(url));
    } else {
      panic!("Unexpected error {:?}", error);
    }
  }

  #[test]
  fn test_download_multiple_pages() {
    // Sanity test to check that the process is not a one-shot operation
    let urls = vec![
      "https://blog.rust-lang.org/feed.xml",
      "https://github.blog/feed",
    ];
    for url in urls {
      let result = get_page(url);
      assert!(result.is_ok(), "Error for {}: {:?}", url, result);
    }
  }
}
