pub fn shell_sort<T: Ord>(v: &mut [T]) {
    let mut h: usize = 1;
    let lenght = v.len();

    while h < lenght / 3 {
        h = h * 3 + 1;
    }

    while h >= 1 {
        for i in h..lenght {
            let mut j: usize = i;
            while j >= h && v[j] < v[j - h] {
                v.swap(j, j - h);
                j -= h;
            }
        }
        h /= 3;
    }
}

#[cfg(test)]
mod tests {
    use super::shell_sort;

    use quickcheck::QuickCheck;

    fn prop_number_sorted(mut data: Vec<i32>) -> bool {
        shell_sort(&mut data);
        data.windows(2).all(|w| w[0] <= w[1])
    }

    fn prop_number_permutation(data: Vec<i32>) -> bool {
        let mut sorted_data = data.clone();
        shell_sort(&mut sorted_data);

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
        shell_sort(&mut data);
        data.windows(2).all(|w| w[0] <= w[1])
    }

    fn prop_str_permutation(data: Vec<String>) -> bool {
        let mut sorted_data = data.clone();
        shell_sort(&mut sorted_data);

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
