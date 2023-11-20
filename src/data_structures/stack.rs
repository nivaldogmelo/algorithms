use crate::data_structures::linked_list;

pub struct Stack<T> {
    pub val: linked_list::LinkedList<T>,
}

impl Stack<i32> {
    pub fn new() -> Stack<i32> {
	Stack {
	    val: linked_list::LinkedList::<i32>::new(),
	}
    }

    pub fn is_empty(&self) -> bool {
	if self.val.next.is_none() {
	    true
	} else {
	    false
	}
    }

    pub fn pop(&mut self) -> Option<i32> {
	if self.is_empty() {
	    None
	} else {
	    self.val.pop_left()
	}
    }

    pub fn push(&mut self, item: i32) {
	self.val.push_left(item);
    }

    pub fn size(&self) -> usize {
	self.collect().len()
    }

    pub fn collect(&self) -> Vec<i32> {
	self.val.collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push() {
	let mut stack: Stack<i32> = Stack::<i32>::new();

	stack.push(1);
	stack.push(2);
	stack.push(3);

	assert_eq!(stack.collect(), vec![3, 2, 1]);
    }

    #[test]
    fn test_is_empty() {
	let stack: Stack<i32> = Stack::<i32>::new();

	assert_eq!(true, stack.is_empty());
    }

    #[test]
    fn test_pop() {
	let mut stack: Stack<i32> = Stack::<i32>::new();

	stack.push(1);
	stack.push(2);
	stack.push(3);

	assert_eq!(3, stack.pop().unwrap());
	assert_eq!(2, stack.pop().unwrap());
	assert_eq!(1, stack.pop().unwrap());
	assert_eq!(true, stack.pop().is_none());
    }

    // #[test]
    // fn test_size() {
    //	let mut stack: Stack<i32> = Stack::<i3, task::Context>::new();

    //	stack.push(1);
    //	stack.push(2);

    //	assert_eq!(2, stack.size());
    // }
}
