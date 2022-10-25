use std::{fmt::Display, fs, time::SystemTime};

use chrono::{DateTime, Utc};
use futures::{stream, StreamExt};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::Client;
use std::fmt::Write as _;
use tokio::runtime::Handle;

pub use blog::{Blog, Post};
mod blog;
pub mod email;
pub mod logger;
pub mod xml;

use crate::xml::parse_web_feed;

const CONCURRENT_REQUESTS: usize = 10;

/// Downloads all the web feeds specified in `feeds.txt` and converts them to `Blog`s.
pub fn download_blogs(days: i64) -> Vec<Blog> {
  let links = read_feeds();

  let contents = if let Ok(handle) = Handle::try_current() {
    std::thread::spawn(move || handle.block_on(get_blogs(links)))
      .join()
      .expect("Error spawning blog download")
  } else {
    let rt = tokio::runtime::Builder::new_current_thread()
      .enable_all()
      .build()
      .expect("Could not build tokio runtime");

    rt.block_on(get_blogs(links))
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

/// Helper method for [download_blogs](download_blogs).
async fn get_blogs(links: Vec<String>) -> Vec<Option<Blog>> {
  let client = Client::new();
  stream::iter(links)
    .map(|link| {
      let client = &client;
      async move {
        let xml = get_page_async(link.as_str(), client)
          .await
          .map_err(|e| warn!("Error in {}\n{}", link, e))
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

/// Parses links from `feeds.txt`.
///
/// Assumed one link per line. Any text between a `#` and a line end
/// is considered a comment.
pub fn read_feeds() -> Vec<String> {
  let links = fs::read_to_string("feeds.txt")
    .or_else(|_| std::env::var("FEEDS"))
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
  let styling_part1 = "
  <!DOCTYPE html>
  <html lang=\"en\">
      <head>
          <meta charset=\"utf-8\" />
          <meta name=\"viewport\" content=\"width=device-width, initial-scale=1, shrink-to-fit=no\" />
          <link rel=\"stylesheet\" href=\"https://cdn.jsdelivr.net/npm/bootstrap@4.3.1/dist/css/bootstrap.min.css\" integrity=\"sha384-ggOyR0iXCbMQv3Xipma34MD+dH/1fQ784/j6cY/iJTQUOhcWr7x9JvoRxT2MZw1T\" crossorigin=\"anonymous\">
      </head>
      <body>
          <nav class=\"navbar navbar-expand-lg navbar-dark bg-dark text-light\">
            <a class=\"navbar-brand\">Rss2Email Feed</a>
          </nav>
          <div class=\"container\">
              <div class=\"text-left mt-5\">
".to_string();

  let mut res = format!("<h1>Rss2Email - {}</h1>", Utc::now().date());

  for blog in blogs {
    let mut tmp = format!("<h2>{}</h2><ul>", blog.title);
    for post in &blog.posts {
      let _ = write!(tmp, "<li><a href=\"{}\">{}</a></li>", post.link, post.title);
    }
    tmp.push_str("</ul>");
    res.push_str(&tmp);
  }

  let styling_part2: String = "
    </div>
    </div>
    
    <!-- Bootstrap core JS-->
    <script src=\"https://cdn.jsdelivr.net/npm/bootstrap@5.1.3/dist/js/bootstrap.bundle.min.js\"></script>
  </body>
  </html>
".to_string();

  let mut res2: String = String::new();
  res2.push_str(&styling_part1);
  res2.push_str(&res);
  res2.push_str(&styling_part2);

  res2
}

/// Returns true if the passed date is within `n` days from the current date.
fn within_n_days(n: i64, date: &DateTime<Utc>) -> bool {
  let today = Utc::now();
  let date = date.with_timezone(&Utc);
  (today - date).num_days() <= n
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
pub async fn get_page_async(url: &str, client: &Client) -> Result<String, DownloadError> {
  let response = client
    .get(url)
    .header(
      "Accept",
      "application/xml, text/xml, application/rss+xml, application/atom+xml",
    )
    .header("User-Agent", "Rss2Email");
  let response = response.send().await?;

  let content_type = response
    .headers()
    .get(reqwest::header::CONTENT_TYPE)
    .ok_or_else(|| DownloadError::Custom("No content type header found on request.".to_string()))?
    .to_str()
    .map_err(|_e| DownloadError::Custom("Content Type parsing error".to_string()))?
    .split(';')
    .collect::<Vec<&str>>()[0]
    .to_owned();

  if !is_supported_content(&content_type) {
    return Err(DownloadError::Custom(format!(
      "Invalid content {} for {}",
      content_type.as_str(),
      url
    )));
  }

  response
    .text()
    .await
    .map_err(|_e| DownloadError::Custom("Body decode error".to_string()))
}

/// Helper function that times and prints the elapsed execution time
/// of `F` if ran in debug mode.
///
/// # Usage
///
/// ```
/// use rss2email_lib::*;
/// let blogs: Vec<Blog> = time_func(|| download_blogs(7), "download_blogs");
/// ```
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

/// Represents anything that could go wrong when downloading the web feeds.
#[derive(Debug)]
pub enum DownloadError {
  /// Wrapper for [reqwest::Error].
  Reqwest(Box<reqwest::Error>),
  HeaderString(Box<http::header::ToStrError>),
  Io(std::io::Error),
  Custom(String),
}

impl Display for DownloadError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match &self {
      Self::Reqwest(e) => write!(f, "{}", e),
      Self::HeaderString(e) => write!(f, "{}", e),
      Self::Io(e) => write!(f, "{}", e),
      Self::Custom(e) => write!(f, "{}", e),
    }
  }
}

impl From<std::io::Error> for DownloadError {
  fn from(error: std::io::Error) -> Self {
    Self::Io(error)
  }
}

impl From<reqwest::Error> for DownloadError {
  fn from(error: reqwest::Error) -> Self {
    Self::Reqwest(Box::new(error))
  }
}

impl From<http::header::ToStrError> for DownloadError {
  fn from(error: http::header::ToStrError) -> Self {
    Self::HeaderString(Box::new(error))
  }
}
