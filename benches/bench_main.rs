use criterion::criterion_main;

mod benchmarks;

criterion_main! {
  benchmarks::map_to_html::map_to_html_bench,
  benchmarks::download_blogs::download_blogs_bench,
  benchmarks::get_page::get_page_bench,
}
