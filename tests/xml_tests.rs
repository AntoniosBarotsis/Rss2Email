use chrono::{DateTime, Utc};
use rss2email_lib::{
  xml::{parse_web_feed, error::ParserError},
  Blog, Post,
};

fn read_file(dir_name: &str, file_name: &str) -> String {
  use std::fs;
  use std::path::PathBuf;
  let mut file_path = PathBuf::from("test-data");
  file_path.push(dir_name);
  file_path.push(file_name);

  fs::read_to_string(file_path).expect("Cannot read feed ")
}

fn read_rss(file_name: &str) -> String {
  read_file("rss-feeds", file_name)
}

fn read_atom(file_name: &str) -> String {
  read_file("atom-feeds", file_name)
}

fn post_date(value: &str) -> DateTime<Utc> {
  value
    .parse::<DateTime<Utc>>()
    .unwrap_or_else(|_| panic!("Invalid date {}", value))
}

#[test]
fn test_parse_brief_single_entry_atom() {
  let content = read_atom("brief-single-entry.xml");
  let blog = parse_web_feed(&content).expect("Parsed content");

  assert_eq!(
    blog,
    Blog {
      title: "Example Feed".into(),
      last_build_date: post_date("2003-12-13T18:30:02+00:00"),
      posts: vec![Post {
        title: "Atom-Powered Robots Run Amok".into(),
        link: "http://example.org/2003/12/13/atom03".into(),
        description: Some("Some text.".into()),
        last_build_date: post_date("2003-12-13T18:30:02+00:00"),
      }],
    }
  );
}

#[test]
fn test_parse_complex_single_entry_atom() {
  let content = read_atom("complex-single-entry.xml");
  let blog = parse_web_feed(&content).expect("Parsed content");

  // The chosen link is the .../2005/04/02/atom because it is the first in the list
  assert_eq!(
    blog,
    Blog {
      title: "dive into mark".into(),
      last_build_date: post_date("2005-07-31T12:29:29+00:00"),
      posts: vec![Post {
        title: "Atom draft-07 snapshot".into(),
        link: "http://example.org/2005/04/02/atom".into(),
        description: None,
        last_build_date: post_date("2005-07-31T12:29:29+00:00"),
      }],
    }
  );
}

#[test]
fn test_parse_atom_without_entry() {
  let content = read_atom("no-entries.xml");
  let result = parse_web_feed(&content);
  assert!(result.is_err());

  let is_empty_feed_error = match result.expect_err("Should error") {
    ParserError::Parse(p) => p.contains("Empty feed"),
    ParserError::Date(_) => false,
  };

  assert!(is_empty_feed_error);
}

#[test]
fn test_parse_atom_with_many_entries() {
  let content = read_atom("multi-entries.xml");
  let blog = parse_web_feed(&content).expect("Parsed content");

  let first_date = post_date("2022-09-18T21:00:00+00:00");
  let second_date = post_date("2022-10-21T21:10:00+00:00");
  assert_eq!(
    blog,
    Blog {
      title: "Multi-Entries Feed".into(),
      last_build_date: second_date,
      posts: vec![
        Post {
          title: "First title".into(),
          link: "http://awesome.com/link1.html".into(),
          description: Some("First content".into()),
          last_build_date: first_date,
        },
        Post {
          title: "Second title".into(),
          link: "http://com.net/why-not.html".into(),
          description: None,
          last_build_date: second_date,
        }
      ],
    }
  );
}

#[test]
fn test_parse_atom_text_with_html_tags() {
  let content = read_atom("entry-with-html.xml");
  let blog = parse_web_feed(&content).expect("Parsed content");

  let date = post_date("2022-09-18T21:00:00+00:00");
  assert_eq!(
    blog,
    Blog {
      title: "Multi-Entries Feed".into(),
      last_build_date: date,
      posts: vec![Post {
        title: "<b>Star</b> City".into(),
        link: "http://link.com".into(),
        description: Some(
          "How did it work? <i>Details</i> <a href=\"http://liftoff.msfc.nasa.gov\">here</a>"
            .into()
        ),
        last_build_date: date,
      }],
    }
  );
}

// Ignored:
// - multiple feeds in a single document

#[test]
fn test_parse_rss_data() {
  let content = read_rss("self-rss.xml");
  let _blog = parse_web_feed(&content).expect("Parsed content");
}

#[test]
fn test_parse_rss_0_91() {
  let content = read_rss("sample-0_91.xml");
  let result = parse_web_feed(&content);
  assert!(result.is_err());
}

#[test]
fn test_parse_rss_0_92() {
  let content = read_rss("sample-0_92.xml");
  let result = parse_web_feed(&content);
  assert!(result.is_err());
}

#[test]
fn test_parse_rss_2() {
  let content = read_rss("sample-2.xml");
  let blog = parse_web_feed(&content).expect("Parsed content");
  assert_eq!(
    blog,
    Blog {
      title: "Liftoff News".into(),
      last_build_date: post_date("2003-06-10T04:00:00+00:00"),
      posts: vec![Post {
        title: "Star City".into(),
        link:"http://liftoff.msfc.nasa.gov/news/2003/news-starcity.asp".into(),
        description: Some("How do Americans get ready to work with Russians aboard the International Space Station? They take a crash course in culture, language and protocol at Russia's <a href=\"http://howe.iki.rssi.ru/GCTC/gctc_e.htm\">Star City</a>.".into()),
        last_build_date: post_date("2003-06-03T09:39:21+00:00"),
      },
      // Sky watchers post ignored as not containing link
       Post {
        title: "The Engine That Does More".into(),
        link:"http://liftoff.msfc.nasa.gov/news/2003/news-VASIMR.asp".into(),
        description: Some("Before man travels to Mars, NASA hopes to design new engines that will let us fly through the Solar System more quickly.  The proposed VASIMR engine would do that.".into()),
        last_build_date: post_date("2003-05-27T08:37:32+00:00"),
      },
       Post {
        title: "Astronauts' Dirty Laundry".into(),
        link:"http://liftoff.msfc.nasa.gov/news/2003/news-laundry.asp".into(),
        description: Some("Compared to earlier spacecraft, the International Space Station has many luxuries, but laundry facilities are not one of them.  Instead, astronauts have other options.".into()),
        last_build_date: post_date("2003-05-20T08:56:02+00:00"),
      }],
    });
}

#[test]
fn test_parse_rss_text_with_html_tags() {
  let content = read_rss("v2-with-html.xml");
  let blog = parse_web_feed(&content).expect("Parsed content");
  assert_eq!(
    blog,
    Blog {
      title: "Liftoff News".into(),
      last_build_date: post_date("2003-06-10T04:00:00+00:00"),
      posts: vec![Post {
        title: "<b>Star</b> City".into(),
        link: "http://liftoff.msfc.nasa.gov/news/2003/news-starcity.asp".into(),
        description: Some(
          "How did it work? <i>Details</i> <a href=\"http://liftoff.msfc.nasa.gov\">here</a>"
            .into()
        ),
        last_build_date: post_date("2003-06-03T09:39:21+00:00"),
      }],
    }
  );
}

#[test]
fn test_parse_rss_without_items() {
  let content = read_rss("v2-without-items.xml");
  let blog = parse_web_feed(&content).expect("Parsed content");
  assert_eq!(
    blog,
    Blog {
      title: "NoNews".into(),
      last_build_date: post_date("2003-06-10T04:00:00+00:00"),
      posts: vec![]
    }
  );
}

/// Tests that entries without links are correctly ignored
/// Not having a link means that there is no place to redirect to to read the story
#[test]
fn test_parse_rss_entry_without_link() {
  let content = read_rss("v2-without-link.xml");
  let blog = parse_web_feed(&content).expect("Parsed content");
  assert_eq!(
    blog,
    Blog {
      title: "Liftoff News".into(),
      last_build_date: post_date("2003-06-10T04:00:00+00:00"),
      posts: vec![
        Post {
          title: "Star City".into(),
          link: "http://abc.com".into(),
          description: None,
          last_build_date: post_date("2003-06-03T09:39:21+00:00"),
        },
        Post {
          title: "Planet City".into(),
          link: "http://def.com".into(),
          description: Some("def".into()),
          last_build_date: post_date("2003-06-03T09:39:21+00:00"),
        }
      ],
    }
  );
}

#[test]
fn test_parse_rss_entry_without_title_and_short_description() {
  let content = read_rss("v2-without-title-short-desc.xml");
  let blog = parse_web_feed(&content).expect("Parsed content");
  assert_eq!(
    blog,
    Blog {
      title: "Liftoff News".into(),
      last_build_date: post_date("2003-06-10T04:00:00+00:00"),
      posts: vec![Post {
        title: "Liftoff at Star City".into(),
        link: "http://liftoff.msfc.nasa.gov".into(),
        description: None,
        last_build_date: post_date("2003-06-03T09:39:21+00:00"),
      }],
    }
  );
}

#[test]
fn test_parse_rss_entry_without_title_and_long_description() {
  let content = read_rss("v2-without-title-long-desc.xml");
  let blog = parse_web_feed(&content).expect("Parsed content");
  assert_eq!(
    blog,
    Blog {
      title: "Liftoff News".into(),
      last_build_date: post_date("2003-06-10T04:00:00+00:00"),
      posts: vec![Post {
        title: "How do Americans get ready to work with Russians a...".into(),
        link: "http://liftoff.msfc.nasa.gov".into(),
        description: Some(
          "How do Americans get ready to work with Russians aboard the International Space Station? They take a crash course in culture, language and protocol at Russia's Star City."
            .into()
        ),
        last_build_date: post_date("2003-06-03T09:39:21+00:00"),
      }],
    }
  );
}
