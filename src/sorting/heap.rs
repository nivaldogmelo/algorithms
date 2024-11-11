use crate::data_structures::BinaryHeap;

pub fn heap_sort<T>(v: &mut [T]) -> BinaryHeap<T>
where
    T: PartialOrd + std::default::Default + std::marker::Copy,
{
    let mut size = v.len();
    let list_size = v.len();
    let mut heap = BinaryHeap::new_from_array(v);

    while size > 1 {
	heap.val.swap(1, size);
	size -= 1;
	heap.sink_to_n(1, size);
    }

    (0..list_size).for_each(|i| v[i] = heap.val[i + 1]);
    heap
}

#[cfg(test)]
mod tests {
    use super::heap_sort;

    use quickcheck::QuickCheck;

    fn prop_number_sorted(mut data: Vec<i32>) -> bool {
	heap_sort(&mut data);
	data.windows(2).all(|w| w[0] <= w[1])
    }

    #[test]
    fn quickcheck_sorted() {
	QuickCheck::new()
	    .tests(1000)
	    .quickcheck(prop_number_sorted as fn(Vec<i32>) -> bool);
    }

    fn prop_number_permutation(data: Vec<i32>) -> bool {
	let mut sorted_data = data.clone();
	heap_sort(&mut sorted_data);

	let mut original = data.clone();
	original.sort();

	sorted_data == original
    }

    #[test]
    fn quickcheck_permutation() {
	QuickCheck::new()
	    .tests(1000)
	    .quickcheck(prop_number_permutation as fn(Vec<i32>) -> bool);
    }
}
