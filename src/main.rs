mod email;
mod logger;

use crate::email::email_provider::{get_email_provider, EmailProvider};
use dotenvy::dotenv;
use env_logger::Env;
use rss2email_lib::{download_blogs, html_title, map_to_html, time_func};

/// The core logic of the main function. This should be called regardless of where
/// you are running the project at.
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

  if posts_amt == 0
    && std::env::var("SKIP_IF_NO_NEW_POSTS").is_ok_and(|v| v.to_lowercase() == "true")
  {
    info!("No posts found and SKIP_IF_NO_NEW_POSTS was set to true, exiting...");
    return Ok(());
  }

  let html = if blogs.is_empty() {
    format!("{}\nNo new posts were found. You can set \"SKIP_IF_NO_NEW_POSTS\" to \"true\" to avoid sending this email.", html_title())
  } else {
    map_to_html(&blogs)
  };

  if cfg!(debug_assertions) {
    info!("{}", html);
  } else {
    // Only load email related variables if ran on release
    let sender_address = std::env::var("EMAIL_ADDRESS").expect("EMAIL_ADDRESS must be set.");
    let recipient_addresses =
      std::env::var("RECIPIENT_ADDRESSES").expect("RECIPIENT_ADDRESSES must be set");

    let recipient_addresses = recipient_addresses.split(',').collect::<Vec<&str>>();
    let subject = std::env::var("SUBJECT")
      .expect("SUBJECT must be set.")
      .replace("$POST_COUNT", &posts_amt.to_string());

    if let Err(e) = get_email_provider()
      .map(|provider| provider.send_email(&sender_address, recipient_addresses, &subject, &html))?
    {
      error!("{}", e);
    };
  }

  Ok(())
}

/// Calls [`core_main`].
#[cfg(not(feature = "aws-lambda"))]
fn main() -> Result<(), String> {
  core_main()
}

/// Calls [`aws_lambda::lambda_wrapper()`].
#[cfg(feature = "aws-lambda")]
fn main() -> Result<(), aws_lambda::LambdaErr> {
  aws_lambda::lambda_wrapper()
}

/// Contains necessary boiler-plate that allows the project to run on AWS Lambda.
#[cfg(feature = "aws-lambda")]
mod aws_lambda {
  use crate::core_main;
  use lambda_runtime::{run, service_fn, Error, LambdaEvent};
  use rss2email_lib::warn;
  use serde::Deserialize;
  pub type LambdaErr = Error;

  #[derive(Deserialize)]
  struct Request {}

  #[allow(clippy::unused_async, clippy::no_effect_underscore_binding)]
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
