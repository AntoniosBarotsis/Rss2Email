use criterion::criterion_main;

mod benchmarks;

criterion_main! {
  benchmarks::map_to_html::map_to_html_bench
}
