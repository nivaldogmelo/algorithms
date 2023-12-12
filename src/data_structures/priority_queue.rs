use crate::sorting::td_merge_sort;

pub struct PriorityQueue {
    pub val: Vec<i64>,
}

impl PriorityQueue {
    pub fn new(val: Vec<i64>) -> PriorityQueue {
	PriorityQueue { val }
    }

    pub fn swim(&mut self, mut k: usize) {
	while k > 1 && self.val[k] > self.val[k / 2] {
	    self.val.swap(k, k / 2);
	    k = k / 2
	}
    }

    pub fn sink(&mut self, mut k: usize) {
	println!("{:?}", self.val);
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

    pub fn insert(&mut self, n: i64) {
	self.val.push(n);
	self.swim(self.val.len() - 1);
    }

    pub fn del_max(&mut self) {
	let size = self.val.len() - 1;
	self.val.swap(1, size);
	self.val.pop();
	self.sink(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
	let expected = vec![0, 20, 19, 18, 14, 16, 15, 1, 5, 9, 7, 8];
	let val = vec![0, 20, 16, 18, 14, 8, 15, 1, 5, 9, 7];
	let mut pq = PriorityQueue::new(val);
	pq.insert(19);

	assert_eq!(expected, pq.val);
    }

    #[test]
    fn test_del_max() {
	let expected = vec![0, 19, 16, 18, 14, 8, 15, 1, 5, 9, 7];
	let value = vec![0, 20, 19, 18, 14, 16, 15, 1, 5, 9, 7, 8];

	let mut pq = PriorityQueue::new(value);
	pq.del_max();

	assert_eq!(expected, pq.val);
    }
}
