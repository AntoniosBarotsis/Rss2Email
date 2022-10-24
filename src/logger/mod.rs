//! This file contains wrapper macros for the `env_logger` crate.
//! I specifically wanted to wrap around `warn` as I wanted all warnings
//! to panic if they occured in the Github Actions workflow.

/// Calls [`log::info!`].
#[macro_export]
macro_rules! info {
  ( $($arg:tt)+ ) => {{
    log::info!($($arg)+)
  }}
}

/// Calls [`log::warn!`] if executed outside of Github Actions. If it is executed inside
/// of GA, it instead panics. This is done to make sure that no warnings are missed during a
/// workflow run. GA sets an environment variable `CI=TRUE` which is how I determine if this
/// should panic or not.
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

/// Calls [`log::error!`].
#[macro_export]
macro_rules! error {
  ( $($arg:tt)+ ) => {{
    log::error!($($arg)+)
  }}
}
