extern crate rand;

pub fn quick_sort<T: Ord>(v: &mut [T]) {
    let size = v.len();
    if size <= 1 {
	return;
    }
    sort(v, 0, size - 1);
}

fn sort<T: Ord>(v: &mut [T], lo: usize, hi: usize) {
    if hi <= lo {
	return;
    }

    let j = partition(v, lo, hi);
    if j == 0 {
	sort(v, j + 1, hi);
    } else if j == hi {
	sort(v, lo, j - 1);
    } else {
	sort(v, lo, j - 1);
	sort(v, j + 1, hi);
    }
}

fn partition<T: Ord>(v: &mut [T], lo: usize, hi: usize) -> usize {
    let mut j = hi;
    let mut i = lo + 1;
    let part_item = lo;

    loop {
	while v[i] < v[part_item] {
	    if i >= j {
		break;
	    }
	    i += 1;
	}

	while v[j] > v[part_item] {
	    if j <= i {
		break;
	    }
	    j -= 1;
	}

	if v[i] == v[j] {
	    if i < j {
		i += 1;
		j -= 1;
	    } else {
		break;
	    }
	}

	v.swap(i, j);
    }

    if v[i] <= v[part_item] {
	v.swap(part_item, i);
	i
    } else {
	v.swap(part_item, i - 1);
	i - 1
    }
}

#[cfg(test)]
mod tests {
    use super::quick_sort;

    use quickcheck::QuickCheck;

    fn prop_number_sorted(mut data: Vec<i32>) -> bool {
	quick_sort(&mut data);
	data.windows(2).all(|w| w[0] <= w[1])
    }

    fn prop_number_permutation(data: Vec<i32>) -> bool {
	let mut sorted_data = data.clone();
	quick_sort(&mut sorted_data);

	let mut original = data.clone();
	original.sort();

	sorted_data == original
    }

    #[test]
    fn quickcheck_numbers() {
	QuickCheck::new()
	    .tests(1000)
	    .quickcheck(prop_number_sorted as fn(Vec<i32>) -> bool);

	QuickCheck::new()
	    .tests(1000)
	    .quickcheck(prop_number_permutation as fn(Vec<i32>) -> bool);
    }

    fn prop_str_sorted(mut data: Vec<String>) -> bool {
	quick_sort(&mut data);
	data.windows(2).all(|w| w[0] <= w[1])
    }

    fn prop_str_permutation(data: Vec<String>) -> bool {
	let mut sorted_data = data.clone();
	quick_sort(&mut sorted_data);

	let mut original = data.clone();
	original.sort();

	sorted_data == original
    }

    #[test]
    fn quickcheck_str() {
	QuickCheck::new()
	    .tests(1000)
	    .quickcheck(prop_str_sorted as fn(Vec<String>) -> bool);

	QuickCheck::new()
	    .tests(1000)
	    .quickcheck(prop_str_permutation as fn(Vec<String>) -> bool);
    }
}
