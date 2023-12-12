extern crate rand;

use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn quick_sort(v: &mut Vec<i64>) {
    v.shuffle(&mut thread_rng());
    sort(v, 0, v.len() - 1);
}

fn sort(v: &mut Vec<i64>, lo: usize, hi: usize) {
    if hi <= lo {
	return;
    }

    let mut j = partition(v, lo, hi);
    if j == 0 {
	j += 1;
    }
    sort(v, lo, j - 1);
    sort(v, j + 1, hi);
}

fn partition(v: &mut Vec<i64>, lo: usize, hi: usize) -> usize {
    let mut j = hi;
    let mut i = lo + 1;
    let part_item = v[lo];

    loop {
	while v[i] < part_item {
	    i += 1;
	    if i == hi {
		break;
	    }
	}

	while v[j] > part_item {
	    j -= 1;
	    if j == lo {
		break;
	    }
	}

	if i >= j {
	    break;
	}

	v.swap(i, j);
    }

    v.swap(lo, j);
    j
}

pub fn quick_sort3(v: &mut Vec<i64>) {
    sort3(v, 0, v.len() - 1);
}

fn sort3(v: &mut Vec<i64>, lo: usize, hi: usize) {
    if hi <= lo {
	return;
    }

    let mut lt = lo;
    let mut i = lo + 1;
    let mut gt = hi;
    let part_item = v[lo];

    while i <= gt {
	let cmp = v[i] - part_item;

	if cmp < 0 {
	    lt += 1;
	    i += 1;
	    v.swap(lo, i);
	} else if cmp > 0 {
	    gt -= 1;
	    v.swap(i, gt);
	} else {
	    i += 1;
	}
    }
    sort(v, lo, lt - 1);
    sort(v, gt + 1, hi);
}
