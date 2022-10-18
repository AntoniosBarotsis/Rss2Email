use crate::email::email_provider::{get_email_provider, EmailProvider};
use dotenvy::dotenv;
use env_logger::Env;
use log::{error, info, warn};
use rss2email::{download_blogs, map_to_html, time_func};

mod email;

fn core_main() -> Result<(), String> {
  if env_logger::Builder::from_env(Env::default().default_filter_or("info"))
    .try_init()
    .is_ok()
  {};

  let _env = dotenv();
  let days_default: i64 = 7;

  let days = std::env::var("DAYS").map_or(days_default, |v| {
    v.parse::<i64>()
      .map_err(|e| {
        warn!("Invalid number for days, using defaults! error: {e}");
      })
      .unwrap_or(days_default)
  });

  info!("Days set to {days}",);

  let blogs = time_func(|| download_blogs(days), "download_blogs");

  let posts_amt = blogs.iter().flat_map(|x| &x.posts).count();
  info!(
    "Downloaded {} blogs with {} posts total.",
    blogs.len(),
    posts_amt
  );

  let html = map_to_html(&blogs);

  if cfg!(debug_assertions) {
    info!("{}", html);
  } else {
    // Only load email related variables if ran on release
    let address = std::env::var("EMAIL_ADDRESS").expect("EMAIL_ADDRESS must be set.");

    if let Err(e) = get_email_provider().map(|provider| provider.send_email(&address, &html))? {
      error!("{}", e);
    };
  }

  Ok(())
}

#[cfg(not(feature = "aws-lambda"))]
fn main() -> Result<(), String> {
  core_main()
}

#[cfg(feature = "aws-lambda")]
fn main() -> Result<(), aws_lambda::LambdaErr> {
  aws_lambda::lambda_wrapper()
}

#[cfg(feature = "aws-lambda")]
mod aws_lambda {
  use crate::core_main;
  use lambda_runtime::{run, service_fn, Error, LambdaEvent};
  use serde::Deserialize;
  pub type LambdaErr = Error;
  use log::warn;

  #[derive(Deserialize)]
  struct Request {}

  #[allow(clippy::unused_async)]
  async fn function_handler(_event: LambdaEvent<Request>) -> Result<(), Error> {
    // Extract some useful information from the request
    let _res = core_main().map_err(|x| warn!("{}", x));
    Ok(())
  }

  #[tokio::main]
  pub async fn lambda_wrapper() -> Result<(), Error> {
    tracing_subscriber::fmt()
      .with_max_level(tracing::Level::INFO)
      // disable printing the name of the module in every log line.
      .with_target(false)
      // disabling time is handy because CloudWatch will add the ingestion time.
      .without_time()
      .init();

    run(service_fn(function_handler)).await
  }
}
