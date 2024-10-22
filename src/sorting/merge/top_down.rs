pub fn merge_sort<T: Ord + Copy>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }

    sort(v, 0, v.len() - 1);
}

fn sort<T: Ord + Copy>(v: &mut [T], lo: usize, hi: usize) {
    if hi <= lo {
        return;
    }

    let mid = lo + (hi - lo) / 2;
    sort(v, lo, mid);
    sort(v, mid + 1, hi);
    merge(v, lo, mid, hi);
}

fn merge<T: Ord + Copy>(v: &mut [T], lo: usize, mid: usize, hi: usize) {
    let mut left: Vec<T> = Vec::new();
    let mut right: Vec<T> = Vec::new();

    let mut i = 0;
    let mut j = 0;

    (lo..mid + 1).for_each(|k| {
        left.push(v[k]);
    });

    (mid + 1..hi + 1).for_each(|k| {
        right.push(v[k]);
    });

    (lo..(hi + 1)).for_each(|k| {
        if i >= left.len() {
            v[k] = right[j];
            j += 1;
        } else if (j >= right.len()) || (left[i] < right[j]) {
            v[k] = left[i];
            i += 1;
        } else {
            v[k] = right[j];
            j += 1;
        }
    });
}

#[cfg(test)]
mod tests {
    use super::merge_sort;

    use quickcheck::QuickCheck;

    fn prop_number_sorted(mut data: Vec<i32>) -> bool {
        merge_sort(&mut data);
        data.windows(2).all(|w| w[0] <= w[1])
    }

    fn prop_number_permutation(data: Vec<i32>) -> bool {
        let mut sorted_data = data.clone();
        merge_sort(&mut sorted_data);

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
}
