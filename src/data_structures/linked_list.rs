use std::ptr::NonNull;

pub struct LinkedList<T> {
    pub val: Option<T>,
    pub next: Option<NonNull<LinkedList<T>>>,
}

impl LinkedList<i32> {
    // # Creates an empty LinkedList that may hold i32 values
    //
    // # Example
    // ```
    // let list = linked_list::LinkedList::<i32>::new();
    // ```
    pub fn new() -> LinkedList<i32> {
	LinkedList {
	    val: None,
	    next: None,
	}
    }

    pub fn push_left(&mut self, x: i32) {
	// allocate on the heap
	let node = Box::new(LinkedList::<i32> {
	    val: Some(x),
	    next: None,
	});

	if self.next.is_none() {
	    // our list is empty, take control over the raw pointer
	    let pter: NonNull<LinkedList<i32>> = Box::leak(node).into();

	    // head is now the pointer
	    self.next = Some(pter);
	} else {
	    // our list already has an element. Take control over the raw pointer
	    // as mutable
	    let mut pter: NonNull<LinkedList<i32>> = Box::leak(node).into();
	    unsafe {
		// new node should point to current head
		pter.as_mut().next = self.next;
	    }

	    // head is now the pointer
	    self.next = Some(pter);
	}
    }

    pub fn push_right(&mut self, x: i32) {
	// allocate on the heap
	let node = Box::new(LinkedList::<i32> {
	    val: Some(x),
	    next: None,
	});

	if self.next.is_none() {
	    // our list is empty, take control over the raw pointer
	    let pter: NonNull<LinkedList<i32>> = Box::leak(node).into();

	    // head is now the pointer
	    self.next = Some(pter);
	} else {
	    // our list already has an element. Take control over the raw pointer
	    // as mutable
	    let pter: NonNull<LinkedList<i32>> = Box::leak(node).into();

	    let mut node = self.next.unwrap();
	    // Go until last node
	    unsafe {
		while node.as_mut().next.is_some() {
		    node = node.as_mut().next.unwrap();
		}
		node.as_mut().next = Some(pter);
	    }
	}
    }

    pub fn pop_left(&mut self) -> Option<i32> {
	// empty list
	if self.next.is_none() {
	    return None;
	} else {
	    let mut next = self.next.unwrap();

	    // list has only one element
	    let only_one: bool = unsafe { next.as_mut().next.is_none() };
	    if only_one == true {
		// pull the next element from the raw pointer
		// and store it in a box, so memory free can work correctly
		let next_box = unsafe { Box::from_raw(next.as_ptr()) };
		self.next = None;
		next_box.val
	    } else {
		// list has two or more elements
		let next_of_next = unsafe { next.as_mut().next };
		let next_box = unsafe { Box::from_raw(next.as_ptr()) };
		self.next = next_of_next;
		next_box.val
	    }
	}
    }

    pub fn pop_right(&mut self) -> Option<i32> {
	// empty list
	if self.next.is_none() {
	    return None;
	} else {
	    let mut node = self.next.unwrap();
	    let val: i32;

	    // list has only one element
	    let only_one: bool = unsafe { node.as_mut().next.is_none() };
	    if only_one == true {
		// pull the next element from the raw pointer
		// and store it in a box, so memory free can work correctly
		let next_box = unsafe { Box::from_raw(node.as_ptr()) };
		self.next = None;
		next_box.val
	    } else {
		let mut old_node: NonNull<LinkedList<i32>>;
		// Go until last node
		unsafe {
		    old_node = node;
		    while node.as_mut().next.is_some() {
			old_node = node;
			node = node.as_mut().next.unwrap();
		    }

		    old_node.as_mut().next = None;
		    val = node.as_mut().val.unwrap();
		}

		Some(val)
	    }
	}
    }

    pub fn collect(&self) -> Vec<i32> {
	let mut result = Vec::<i32>::new();
	if self.next.is_none() {
	    return result;
	}

	let mut node = self.next.unwrap();
	unsafe {
	    result.push(node.as_mut().val.unwrap());
	    while node.as_mut().next.is_some() {
		node = node.as_mut().next.unwrap();
		result.push(node.as_mut().val.unwrap());
	    }
	}

	return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_left() {
	let mut list: LinkedList<i32> = LinkedList::<i32>::new();

	list.push_left(1);
	list.push_left(2);
	list.push_left(3);
	list.push_left(4);

	assert_eq!(list.collect(), vec![4, 3, 2, 1]);
    }

    #[test]
    fn test_push_right() {
	let mut list: LinkedList<i32> = LinkedList::<i32>::new();

	list.push_right(1);
	list.push_right(2);
	list.push_right(3);
	list.push_right(4);

	assert_eq!(list.collect(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_pop_left() {
	let mut list: LinkedList<i32> = LinkedList::<i32>::new();

	list.push_left(1);
	list.push_left(2);
	list.push_left(3);
	list.push_left(4);

	let x = list.pop_left();
	assert_eq!(x.unwrap(), 4);

	let x = list.pop_left();
	assert_eq!(x.unwrap(), 3);

	let x = list.pop_left();
	assert_eq!(x.unwrap(), 2);

	let x = list.pop_left();
	assert_eq!(x.unwrap(), 1);

	let x = list.pop_left();
	assert_eq!(x.is_none(), true);
    }

    #[test]
    fn pop_right() {
	let mut list: LinkedList<i32> = LinkedList::<i32>::new();

	list.push_right(1);
	list.push_right(2);
	list.push_right(3);
	list.push_right(4);

	let x = list.pop_right();
	assert_eq!(x.unwrap(), 4);

	let x = list.pop_right();
	assert_eq!(x.unwrap(), 3);

	let x = list.pop_right();
	assert_eq!(x.unwrap(), 2);

	let x = list.pop_right();
	assert_eq!(x.unwrap(), 1);

	let x = list.pop_right();
	assert_eq!(x.is_none(), true);
    }
}
