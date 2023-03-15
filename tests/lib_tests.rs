use rss2email_lib::{get_page_async, Error};
use tokio::test;

use crate::common::create_client;
mod common;

#[test]
async fn test_download_plain_xml() {
  let payload = get_page_async(
    "https://antoniosbarotsis.github.io/index.xml",
    &create_client(),
  )
  .await;
  let content = payload.expect("Downloaded content");
  assert!(content.starts_with("<?xml"));
  assert!(content.contains("<rss"));
  assert!(content.ends_with("</rss>"));
}

#[test]
async fn test_download_xml_for_rss() {
  let payload = get_page_async("https://github.blog/feed", &create_client()).await;
  let content = payload.expect("Downloaded content");
  assert!(content.starts_with("<?xml"));
}

#[test]
async fn test_download_invalid_page() {
  let payload = get_page_async(
    "https://antoniosbarotsis.github.io/ordex.pkxml",
    &create_client(),
  )
  .await;
  assert!(payload.is_err());
}

#[test]
async fn test_download_with_text() {
  // Text should not be confused with xml: here, we received html for a markdown
  let url = "https://raw.githubusercontent.com/AntoniosBarotsis/Rss2Email/cc5b2bee846f9dab8f5787dfcb9a01d963321630/README.md";
  let payload = get_page_async(url, &create_client()).await;
  assert!(payload.is_err());
  let error = payload.expect_err("Should error");
  if let Error::Generic(message) = error {
    assert!(message.contains("Invalid content"));
    assert!(message.contains(url));
  } else {
    panic!("Unexpected error {error:?}");
  }
}

#[test]
async fn test_download_with_an_image() {
  // Using an URL to a specific sha of this github repo to make sure the target remains
  let url = "https://github.com/AntoniosBarotsis/Rss2Email/raw/cc5b2bee846f9dab8f5787dfcb9a01d963321630/assets/res.jpg";
  let payload = get_page_async(url, &create_client()).await;
  assert!(payload.is_err());
  let error = payload.expect_err("Should error");
  if let Error::Generic(message) = error {
    assert!(message.contains("Invalid content"));
    assert!(message.contains(url));
  } else {
    panic!("Unexpected error {error:?}");
  }
}

#[test]
async fn test_download_multiple_pages() {
  // Sanity test to check that the process is not a one-shot operation
  let urls = vec![
    "https://blog.rust-lang.org/feed.xml",
    "https://github.blog/feed",
  ];
  for url in urls {
    let result = get_page_async(url, &create_client()).await;
    assert!(result.is_ok(), "Error for {url}: {result:?}");
  }
}
