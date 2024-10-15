pub fn insertion_sort<T: Ord>(v: &mut [T]) {
    for i in 1..v.len() {
        let mut j = i;
        while j > 0 && v[j] < v[j - 1] {
            v.swap(j, j - 1);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::insertion_sort;

    use quickcheck::QuickCheck;

    fn prop_number_sorted(mut data: Vec<i32>) -> bool {
        insertion_sort(&mut data);
        data.windows(2).all(|w| w[0] <= w[1])
    }

    fn prop_number_permutation(data: Vec<i32>) -> bool {
        let mut sorted_data = data.clone();
        insertion_sort(&mut sorted_data);

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
        insertion_sort(&mut data);
        data.windows(2).all(|w| w[0] <= w[1])
    }

    fn prop_str_permutation(data: Vec<String>) -> bool {
        let mut sorted_data = data.clone();
        insertion_sort(&mut sorted_data);

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
