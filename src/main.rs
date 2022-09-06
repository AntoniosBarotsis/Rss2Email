use crate::util::{download_blogs, map_to_html};
use dotenv::dotenv;
use env_logger::Env;
use log::info;

use crate::{email::sendgrid::send_email, util::time_func};

mod blog;
mod email;
mod util;
mod xml;

fn main() {
  env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

  dotenv().ok();

  let sendgrid_api_key = std::env::var("SENDGRID_API_KEY").expect("SENDGRID_API_KEY must be set.");
  let address = std::env::var("EMAIL_ADDRESS").expect("EMAIL_ADDRESS must be set.");
  let days_default = 7;
  // TODO Email provider as an env var

  let days = match std::env::var("DAYS") {
    Ok(txt) => match txt.parse::<i64>() {
      Ok(n) => n,
      Err(_) => {
        panic!("Days variable is set to \"{}\" which is not a number.", txt)
      }
    },
    Err(_) => days_default,
  };

  info!("Days set to {:?}", days);

  let blogs = time_func(|| download_blogs(days), "download_blogs".to_owned());

  let posts_amt = blogs.iter().flat_map(|x| &x.posts).count();
  info!(
    "Downloaded {} blogs with {} posts total.",
    blogs.len(),
    posts_amt
  );

  let html = map_to_html(&blogs);

  if cfg!(debug_assertions) {
    println!("{}", html);
  } else {
    send_email(address, sendgrid_api_key, html);
  }
}
