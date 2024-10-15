pub fn selection_sort<T: Ord>(v: &mut [T]) {
    let mut min;

    for i in 0..(v.len()) {
        min = i;

        for j in i..v.len() {
            if v[j] < v[min] {
                v.swap(j, min);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::selection_sort;

    use quickcheck::QuickCheck;

    fn prop_number_sorted(mut data: Vec<i32>) -> bool {
        selection_sort(&mut data);
        data.windows(2).all(|w| w[0] <= w[1])
    }

    fn prop_number_permutation(data: Vec<i32>) -> bool {
        let mut sorted_data = data.clone();
        selection_sort(&mut sorted_data);

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
        selection_sort(&mut data);
        data.windows(2).all(|w| w[0] <= w[1])
    }

    fn prop_str_permutation(data: Vec<String>) -> bool {
        let mut sorted_data = data.clone();
        selection_sort(&mut sorted_data);

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
