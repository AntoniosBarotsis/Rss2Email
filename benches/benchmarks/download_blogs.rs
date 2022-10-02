use criterion::{criterion_group, Criterion};
use rss2email::download_blogs;

pub fn criterion_benchmark(c: &mut Criterion) {
  let mut group = c.benchmark_group("download blogs");
  group.bench_function("download blogs", |f| f.iter(|| download_blogs(1)));
  group.finish();
}

criterion_group! {
    name = download_blogs_bench;
    config = Criterion::default().sample_size(10);
    targets = criterion_benchmark
}
