#![deny(rust_2018_idioms)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::pedantic)]

use crate::util::{download_blogs, map_to_html};
use dotenv::dotenv;
use env_logger::Env;
use log::{error, info};

use crate::{email::sendgrid::send_email, util::time_func};

mod blog;
mod email;
mod util;
mod xml;

fn core_main() -> Result<(), String> {
  env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

  dotenv().ok();

  let sendgrid_api_key = std::env::var("SENDGRID_API_KEY").expect("SENDGRID_API_KEY must be set.");
  let address = std::env::var("EMAIL_ADDRESS").expect("EMAIL_ADDRESS must be set.");
  let days_default = 7;
  // TODO Email provider as an env var

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
    Ok(())
  } else {
    send_email(&address, &sendgrid_api_key, &html);
    Ok(())
  }
}

#[cfg(not(feature = "aws-lambda"))]
fn main() -> Result<(), String> {
  core_main()
}

#[cfg(feature = "aws-lambda")]
fn main() -> Result<(), aws_lambda::LambdaErr> {
  aws_lambda::aws_lambda_wrapper()
}

#[cfg(feature = "aws-lambda")]
mod aws_lambda {
  use crate::core_main;
  use lambda_runtime::{run, service_fn, Error, LambdaEvent};
  use serde::Deserialize;
  pub(crate) type LambdaErr = Error;

  #[derive(Deserialize)]
  struct Request {}

  async fn function_handler(_event: LambdaEvent<Request>) -> Result<(), Error> {
    // Extract some useful information from the request
    core_main();
    Ok(())
  }

  #[tokio::main]
  pub(crate) async fn aws_lambda_wrapper() -> Result<(), Error> {
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
