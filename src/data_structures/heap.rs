pub struct BinaryHeap<T: Default> {
    val: Vec<T>,
}

impl<T> BinaryHeap<T>
where
    T: std::cmp::PartialOrd + std::default::Default,
{
    pub fn new() -> Self {
	Self {
	    val: vec![T::default()],
	}
    }

    pub fn new_from_array(val: &mut [T]) -> Self {
	let mut bh = Self::new();

	if val.is_empty() {
	    return bh;
	}

	(0..val.len()).for_each(|i| {
	    let value = std::mem::take(&mut val[i]);

	    bh.val.push(value);
	});

	bh.heapify();
	bh
    }

    fn heapify(&mut self) {
	(self.val.len() / 2..1).for_each(|k| {
	    self.sink(k);
	});
    }

    pub fn swim(&mut self, mut k: usize) {
	while k > 1 && self.val[k] > self.val[k / 2] {
	    self.val.swap(k, k / 2);
	    k /= 2;
	}
    }

    pub fn sink(&mut self, mut k: usize) {
	if k == 0 {
	    k += 1;
	}

	while 2 * k < self.val.len() {
	    let mut j = 2 * k;
	    if j + 1 < self.val.len() && self.val[j] < self.val[j + 1] {
		j += 1;
	    }

	    if self.val[k] > self.val[j] {
		break;
	    }

	    self.val.swap(k, j);

	    k = j;
	}
    }

    pub fn push(&mut self, n: T) {
	self.val.push(n);
    }

    pub fn pop(&mut self) -> Option<T> {
	self.val.pop()
    }

    pub fn root(&self) -> &T {
	if self.val.len() < 2 {
	    return &self.val[0];
	}

	&self.val[1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use quickcheck::QuickCheck;

    fn prop_number_heapified(data: Vec<i32>) -> bool {
	let bh = BinaryHeap::new_from_array(&mut data.clone());
	let mut is_heap = true;

	for k in bh.val.len() / 2..0 {
	    if (2 * k) + 1 < bh.val.len() && bh.val[2 * k + 1] > bh.val[k] {
		is_heap = false;
		break;
	    }

	    if bh.val[2 * k] > bh.val[k] {
		is_heap = false;
		break;
	    }
	}

	is_heap
    }

    fn prop_number_permutation(data: Vec<i32>) -> bool {
	let mut bh = BinaryHeap::new_from_array(&mut data.clone());

	let mut original = vec![i32::default()];
	original.append(&mut data.clone());

	bh.val.sort();
	original.sort();

	bh.val == original
    }

    fn prop_str_heapified(data: Vec<String>) -> bool {
	let bh = BinaryHeap::new_from_array(&mut data.clone());
	let mut is_heap = true;

	for k in bh.val.len() / 2..0 {
	    if (2 * k) + 1 < bh.val.len() && bh.val[2 * k + 1] > bh.val[k] {
		is_heap = false;
		break;
	    }

	    if bh.val[2 * k] > bh.val[k] {
		is_heap = false;
		break;
	    }
	}

	is_heap
    }

    fn prop_str_permutation(data: Vec<String>) -> bool {
	let mut bh = BinaryHeap::new_from_array(&mut data.clone());

	let mut original = vec![String::default()];
	original.append(&mut data.clone());

	bh.val.sort();
	original.sort();

	bh.val == original
    }

    #[test]
    fn quickcheck_heapified() {
	QuickCheck::new()
	    .tests(1000)
	    .quickcheck(prop_number_heapified as fn(Vec<i32>) -> bool);

	QuickCheck::new()
	    .tests(1000)
	    .quickcheck(prop_str_heapified as fn(Vec<String>) -> bool);
    }

    #[test]
    fn quickcheck_permutation() {
	QuickCheck::new()
	    .tests(1000)
	    .quickcheck(prop_number_permutation as fn(Vec<i32>) -> bool);

	QuickCheck::new()
	    .tests(1000)
	    .quickcheck(prop_str_permutation as fn(Vec<String>) -> bool);
    }

    #[test]
    fn test_swim() {
	let expected = vec![0, 20, 19, 18, 14, 16, 15, 1, 5, 9, 7, 8];
	let value = vec![0, 20, 16, 18, 14, 8, 15, 1, 5, 9, 7, 19];

	let mut pq = BinaryHeap::new();
	pq.val = value;
	pq.swim(pq.val.len() - 1);

	assert_eq!(expected, pq.val);
    }

    #[test]
    fn test_sink() {
	let expected = vec![0, 20, 16, 18, 14, 8, 15, 19, 5, 9, 1, 7];
	let value = vec![0, 20, 1, 18, 14, 16, 15, 19, 5, 9, 8, 7];

	let mut pq = BinaryHeap::new();
	pq.val = value;

	pq.sink(2);

	assert_eq!(expected, pq.val);
    }
}
