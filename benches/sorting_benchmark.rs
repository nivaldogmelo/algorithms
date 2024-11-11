extern crate algorithms;
use algorithms::sorting::{
    bottom_up, heap_sort, insertion_sort, quick_sort, selection_sort, shell_sort, top_down,
};

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

    c.bench_function("shell sort", |b| {
	b.iter(|| {
	    let mut subject = data.clone();
	    shell_sort(black_box(&mut subject))
	})
    });

    c.bench_function("top-down mergesort", |b| {
	b.iter(|| {
	    let mut subject = data.clone();
	    top_down::merge_sort(black_box(&mut subject))
	})
    });

    c.bench_function("bottom-up mergesort", |b| {
	b.iter(|| {
	    let mut subject = data.clone();
	    bottom_up::merge_sort(black_box(&mut subject))
	})
    });

    c.bench_function("quicksort", |b| {
	b.iter(|| {
	    let mut subject = data.clone();
	    quick_sort(black_box(&mut subject));
	})
    });

    c.bench_function("heapsort", |b| {
	b.iter(|| {
	    let mut subject = data.clone();
	    heap_sort(black_box(&mut subject));
	})
    });
}
