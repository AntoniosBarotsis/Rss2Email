use crate::email::email_provider::get_email_provider;
use crate::email::email_provider::EmailProvider;
use dotenv::dotenv;
use env_logger::Env;
use log::{error, info};
use std::env;

mod email;

fn core_main() -> Result<(), String> {
  env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

  let _env = dotenv().ok().ok_or("Failed to load .env file")?;

  // Check if using env variable or not -- must delimit using ':', semicolon does not work
  let args: Vec<String> = env::args().collect();    
  let parsed_args: std::str::Split<'_, &str>;
  let mut arg_links: Vec<&str> = Vec::new();
  let mut feed_flag = args.len();
  // check if env vars are present
  if args.len() > 1 {
    parsed_args = args[1].split("*");
    arg_links = parsed_args.collect::<Vec<&str>>(); // contains all feed links
    feed_flag = args.len();
  }

  let sendgrid_api_key = std::env::var("SENDGRID_API_KEY").expect("SENDGRID_API_KEY must be set.");
  let address = std::env::var("EMAIL_ADDRESS").expect("EMAIL_ADDRESS must be set.");
  let days_default = 7;

  let days = match std::env::var("DAYS") {
    Ok(txt) => {
      if let Ok(n) = txt.parse::<i64>() {
        n
      } else {
        error!("Days variable is set to \"{}\" which is not a number.", txt);
        return Err(format!(
          "Days variable is set to \"{}\" which is not a number.",
          txt
        ));
      }
    }
    Err(_) => days_default,
  };

  info!("Days set to {:?}", days);

  let blogs = time_func(|| download_blogs(days, feed_flag, arg_links.clone()), "download_blogs");

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
    let api_key = std::env::var("API_KEY").expect("API_KEY must be set.");
    let address = std::env::var("EMAIL_ADDRESS").expect("EMAIL_ADDRESS must be set.");

    get_email_provider().send_email(&address, &api_key, &html);
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

  #[derive(Deserialize)]
  struct Request {}

  #[allow(clippy::unused_async)]
  async fn function_handler(_event: LambdaEvent<Request>) -> Result<(), Error> {
    // Extract some useful information from the request
    let _res = core_main();
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
