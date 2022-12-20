use std::time::Duration;

use criterion::{criterion_group, BenchmarkId, Criterion, SamplingMode};
use regex::Regex;
use reqwest::Client;
use rss2email_lib::{get_page_async, read_feeds};

pub fn criterion_benchmark(c: &mut Criterion) {
  let mut group = c.benchmark_group("get page");
  group.sampling_mode(SamplingMode::Flat);

  let feeds = read_feeds();

  // Ok, for this it's probably best to check here first https://regex101.com/r/KyY0vd/1
  // but basically, I wanted to get some parts of the URLs that hopefully won't cause
  // collisions in the benchmark IDs.
  let seperator = Regex::new(r"://([a-zA-Z0-9.?-]+)/?([a-zA-Z0-9?-]+)?(?:[^=\n]+)?(?:=(\w+))?")
    .expect("Invalid regex");

  for feed in feeds {
    let client = Client::new();
    let captures = seperator.captures(&feed).unwrap();
    let p1 = captures.get(1).map_or("Regex failed", |m| m.as_str());
    let p2 = captures.get(2).map_or("", |m| m.as_str());
    let p3 = captures.get(3).map_or("", |m| m.as_str());
    let name = p1.to_owned() + "/" + p2 + "/" + p3;

    // This is meant to be for each feed individually so blocking
    // is fine.
    group.bench_with_input(BenchmarkId::from_parameter(name), &feed, |b, feed| {
      b.iter(|| async { get_page_async(feed, &client).await });
    });
  }

  group.finish();
}

criterion_group! {
    name = get_page_bench;
    config = Criterion::default().sample_size(10).measurement_time(Duration::from_secs(4));
    targets = criterion_benchmark
}
