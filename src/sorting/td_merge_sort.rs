pub fn merge_sort(v: &mut Vec<i64>) {
    println!("sort(v, {}, {})", 0, v.len() - 1);
    sort(v, 0, v.len() - 1);
}

fn merge(v: &mut Vec<i64>, lo: usize, mid: usize, hi: usize) {
    let mut left: Vec<i64> = Vec::new();
    let mut right: Vec<i64> = Vec::new();

    let mut i = 0;
    let mut j = 0;

    for k in lo..mid + 1 {
	left.push(v[k]);
    }

    for k in mid + 1..hi + 1 {
	right.push(v[k]);
    }

    for k in lo..(hi + 1) {
	if i >= left.len() {
	    v[k] = right[j];
	    j += 1;
	} else if j >= right.len() {
	    v[k] = left[i];
	    i += 1;
	} else if left[i] < right[j] {
	    v[k] = left[i];
	    i += 1;
	} else {
	    v[k] = right[j];
	    j += 1;
	}
    }
}

fn sort(v: &mut Vec<i64>, lo: usize, hi: usize) {
    if hi <= lo {
	return;
    }

    let mid = lo + (hi - lo) / 2;
    sort(v, lo, mid);
    sort(v, mid + 1, hi);
    merge(v, lo, mid, hi);
}
