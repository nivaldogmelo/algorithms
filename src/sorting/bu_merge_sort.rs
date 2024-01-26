use std::cmp;

pub fn merge_sort<T: Ord + Copy>(v: &mut Vec<T>) {
    sort(v);
}

fn merge<T: Ord + Copy>(v: &mut Vec<T>, lo: usize, mid: usize, hi: usize) {
    let mut left: Vec<T> = Vec::new();
    let mut right: Vec<T> = Vec::new();

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

fn sort<T: Ord + Copy>(v: &mut Vec<T>) {
    let hi = v.len();
    let mut len = 1;

    while len < hi {
	let mut k = 0;
	while k < hi - len {
	    merge(v, k, k + len - 1, cmp::min(k + len + len - 1, hi - 1));
	    k += 2 * len;
	}

	len *= 2;
    }
}

#[cfg(test)]
mod tests {
    use super::sort;

    #[test]
    fn basic() {
	let mut res = vec!["d", "b", "c", "a", "e"];
	let mut clone = res.clone();

	sort(&mut res);
	clone.sort();
	assert_eq!(clone, res);
    }
}
