use blog::{Blog, Post};
use chrono::{DateTime, FixedOffset, Utc};
use env_logger::Env;
use log::{error, info, warn};
use std::io::Write;
use std::{
  fmt::Write as _,
  fs::{self, File},
};
use xml::parse_xml;

mod blog;
mod xml;

fn main() {
  env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

  let days = 7;
  let mut res = String::new();

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

  for content in contents {
    info!(
      "Title: {: ^20} | Date: {} | Posts {}",
      content.title,
      content.last_build_date,
      content.posts.len()
    );

    for post in content.posts {
      _ = write!(
        res,
        "From: {}\nTitle: {}\nDate: {}\nLink: {}\n\n",
        content.title, post.title, post.last_build_date, post.link
      );
    }
  }

  let path = "output.log";
  let mut output = File::create(path).unwrap();
  match write!(output, "{}", res) {
    Ok(_) => info!("Dumped output to {path}"),
    Err(e) => error!("{}", e.to_string()),
  }
}

fn within_n_days(n: i64, date: DateTime<FixedOffset>) -> bool {
  let today = Utc::now();

  let tz = date.timezone();
  let today = today.with_timezone(&tz);
  (today - date).num_days() <= n
}

fn get_page(url: &str) -> Result<String, ureq::Error> {
  let body: String = ureq::get(url)
    .set("Example-Header", "header value")
    .call()?
    .into_string()?;

  Ok(body)
}
