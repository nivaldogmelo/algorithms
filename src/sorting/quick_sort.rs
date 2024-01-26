extern crate rand;

use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn quick_sort<T: Ord + Copy>(v: &mut Vec<T>) {
    v.shuffle(&mut thread_rng());
    sort(v, 0, v.len() - 1);
}

fn sort<T: Ord + Copy>(v: &mut Vec<T>, lo: usize, hi: usize) {
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

fn partition<T: Ord + Copy>(v: &mut Vec<T>, lo: usize, hi: usize) -> usize {
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

#[cfg(test)]
mod tests {
    use super::sort;

    #[test]
    fn basic() {
	let mut res = vec!["d", "b", "c", "a", "e"];
	let mut clone = res.clone();
	let size = res.len() - 1;

	sort(&mut res, 0, size);
	clone.sort();
	assert_eq!(clone, res);
    }
}
