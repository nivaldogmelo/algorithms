extern crate algorithms;
use algorithms::sorting::{insertion_sort, selection_sort};

use criterion::{black_box, Criterion};
use rand::Rng;

fn generate_int_vector(size: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..size)
        .map(|_| rng.gen_range(0..100000))
        .collect::<Vec<i32>>()
}

pub fn benchmark_sorts(c: &mut Criterion) {
    let vec_size = 10_000;

    let data = generate_int_vector(vec_size);

    c.bench_function("selection sort", |b| {
        b.iter(|| {
            let mut subject = data.clone();
            selection_sort(black_box(&mut subject))
        })
    });

    c.bench_function("insertion sort", |b| {
        b.iter(|| {
            let mut subject = data.clone();
            insertion_sort(black_box(&mut subject))
        })
    });
}
