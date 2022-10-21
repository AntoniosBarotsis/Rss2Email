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
      static ref IS_CI: bool = std::env::var("CI").ok().unwrap_or_else(|| "FALSE".to_owned()) == *"TRUE";
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
