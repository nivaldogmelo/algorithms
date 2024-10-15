use criterion::{criterion_group, criterion_main};

mod sorting_benchmark;

criterion_group!(benches, sorting_benchmark::benchmark_sorts);
criterion_main!(benches);
