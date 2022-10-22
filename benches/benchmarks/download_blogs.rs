use criterion::{criterion_group, Criterion};
use lib::download_blogs;

pub fn criterion_benchmark(c: &mut Criterion) {
  c.bench_function("download blogs", |b| b.iter(|| download_blogs(7)));
}

criterion_group! {
    name = download_blogs_bench;
    config = Criterion::default().sample_size(10);
    targets = criterion_benchmark
}
