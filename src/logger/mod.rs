/// This file contains wrapper macros for the `env_logger` crate.
/// I specifically wanted to wrap around `warn` as I wanted all warnings
/// to panic if they occured in the Github Actions workflow

#[macro_export]
macro_rules! info {
  ( $($arg:tt)+ ) => {{
    log::info!($($arg)+)
  }}
}

#[macro_export]
macro_rules! warn {
  ( $($arg:tt)+ ) => {{
    lazy_static::lazy_static!{
      static ref IS_CI: bool = std::env::var("CI").ok().map_or_else(|| "FALSE".to_owned(), |x| x.to_uppercase()) == "TRUE";
    }

    if *IS_CI {
      panic!($($arg)+)
    } else {
      log::warn!($($arg)+)
    }
  }}
}

#[macro_export]
macro_rules! error {
  ( $($arg:tt)+ ) => {{
    log::error!($($arg)+)
  }}
}
