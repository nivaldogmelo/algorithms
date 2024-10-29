use crate::{data_structures::BinaryHeap, domain::PriorityQueue};

pub struct PQ<T: std::default::Default> {
    itens: BinaryHeap<T>,
    size: usize,
}

impl<T: std::cmp::PartialOrd + std::default::Default> PQ<T> {
    pub fn new() -> Self {
	let itens = BinaryHeap::new();

	Self { itens, size: 0 }
    }

    pub fn new_from_array(a: &mut [T]) -> Self {
	let itens = BinaryHeap::new_from_array(a);
	let size = a.len();

	Self { itens, size }
    }
}

impl<T: std::cmp::PartialOrd + std::default::Default> PriorityQueue<T> for PQ<T> {
    fn insert(&mut self, key: T) {
	self.itens.push(key);
	self.itens.swim(self.size);
	self.size += 1;
    }

    fn max(&self) -> &T {
	self.itens.root()
    }

    fn del_max(&mut self) -> Option<T> {
	if self.size < 1 {
	    return None;
	}

	self.itens.val.swap(1, self.size);
	let max = self.itens.pop();

	self.size -= 1;
	self.itens.sink(1);
	Some(max)?
    }

    fn is_empty(&self) -> bool {
	if self.size > 0 {
	    false
	} else {
	    true
	}
    }

    fn size(&self) -> usize {
	self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::QuickCheck;

    fn prop_number_size(mut data: Vec<i32>) -> bool {
	let real_size = data.len();
	let pq = PQ::new_from_array(&mut data);

	real_size == pq.size
    }

    fn prop_str_size(mut data: Vec<String>) -> bool {
	let real_size = data.len();
	let pq = PQ::new_from_array(&mut data);

	real_size == pq.size
    }

    #[test]
    fn quickcheck_permutation() {
	QuickCheck::new()
	    .tests(1000)
	    .quickcheck(prop_number_size as fn(Vec<i32>) -> bool);

	QuickCheck::new()
	    .tests(1000)
	    .quickcheck(prop_str_size as fn(Vec<String>) -> bool);
    }

    fn prop_number_max(mut data: Vec<i32>) -> bool {
	let real_max = data.iter().fold(i32::MIN, |a, b| a.max(*b));

	let mut pq = PQ::new_from_array(&mut data);
	let pq_max = pq.del_max();

	real_max == pq_max.unwrap_or(i32::MIN)
    }

    // fn prop_str_max(data: Vec<String>) -> bool {
    //	let real_max = data.iter().fold("", |a, b| a.max(b));

    //	let mut pq = PQ::new_from_array(&mut data.clone());
    //	let pq_max = pq.del_max();

    //	real_max == pq_max.unwrap_or_default()
    // }

    #[test]
    fn quickcheck_max() {
	QuickCheck::new()
	    .tests(1000)
	    .quickcheck(prop_number_max as fn(Vec<i32>) -> bool);

	// QuickCheck::new()
	//     .tests(1000)
	//     .quickcheck(prop_str_max as fn(Vec<String>) -> bool);
    }
}
