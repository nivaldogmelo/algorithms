pub fn quick_sort(v: &mut Vec<i64>) {
    sort(v, 0, v.len() - 1);
}

fn sort(v: &mut Vec<i64>, lo: usize, hi: usize) {
    if hi <= lo {
	return;
    }

    let j = partition(v, lo, hi);
    if j == 0 {
	return;
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
