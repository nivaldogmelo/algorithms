pub struct List<U, T> {
    head: Link<U, T>,
}

type Link<U, T> = Option<Box<Node<U, T>>>;
type KeyValue<U, T> = (U, T);

struct Node<U, T> {
    key_value: KeyValue<U, T>,
    next: Link<U, T>,
}

pub struct IntoIter<U, T>(List<U, T>);

pub struct Iter<'a, U, T> {
    next: Option<&'a Node<U, T>>,
}

pub struct IterMut<'a, U, T> {
    next: Option<&'a mut Node<U, T>>,
}

impl<U, T> List<U, T> {
    pub fn new() -> Self {
	List { head: None }
    }

    pub fn into_iter(self) -> IntoIter<U, T> {
	IntoIter(self)
    }

    pub fn iter(&self) -> Iter<U, T> {
	Iter {
	    next: self.head.as_deref(),
	}
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, U, T> {
	IterMut {
	    next: self.head.as_deref_mut(),
	}
    }

    pub fn push(&mut self, key: U, elem: T) {
	let node = Box::new(Node {
	    key_value: (key, elem),
	    next: self.head.take(),
	});

	self.head = Some(node);
    }

    pub fn pop(&mut self) -> Option<(U, T)> {
	self.head.take().map(|node| {
	    self.head = node.next;
	    node.key_value
	})
    }
}

impl<U: std::cmp::PartialEq, T: Copy> List<U, T> {
    pub fn update(&mut self, key: U, elem: T) {
	let iter = self.iter_mut();

	for item in iter {
	    if item.0 == key {
		item.1 = elem;
	    }
	}
    }
}

impl<U, T> Iterator for IntoIter<U, T> {
    type Item = (U, T);

    fn next(&mut self) -> Option<Self::Item> {
	self.0.pop()
    }
}

impl<'a, U, T> Iterator for Iter<'a, U, T> {
    type Item = &'a (U, T);

    fn next(&mut self) -> Option<Self::Item> {
	self.next.map(|node| {
	    self.next = node.next.as_deref();
	    &node.key_value
	})
    }
}

impl<'a, U, T> Iterator for IterMut<'a, U, T> {
    type Item = &'a mut (U, T);

    fn next(&mut self) -> Option<Self::Item> {
	self.next.take().map(|node| {
	    self.next = node.next.as_deref_mut();
	    &mut node.key_value
	})
    }
}

impl<U, T> Drop for List<U, T> {
    fn drop(&mut self) {
	let mut curr_link = self.head.take();

	while let Some(mut boxed_node) = curr_link {
	    curr_link = boxed_node.next.take();
	}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
	let mut list = List::new();

	assert_eq!(list.pop(), None);

	list.push("a", 1);
	list.push("c", 2);
	list.push("b", 3);

	assert_eq!(list.pop(), Some(("b", 3)));
	assert_eq!(list.pop(), Some(("c", 2)));

	list.push("e", 4);
	list.push("d", 5);

	assert_eq!(list.pop(), Some(("d", 5)));
	assert_eq!(list.pop(), Some(("e", 4)));
	assert_eq!(list.pop(), Some(("a", 1)));
	assert_eq!(list.pop(), None);
    }

    #[test]
    fn into_iter() {
	let mut list = List::new();
	list.push("a", 1);
	list.push("c", 2);
	list.push("b", 3);

	let mut iter = list.into_iter();
	assert_eq!(iter.next(), Some(("b", 3)));
	assert_eq!(iter.next(), Some(("c", 2)));
	assert_eq!(iter.next(), Some(("a", 1)));
	assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
	let mut list = List::new();
	list.push("a", 1);
	list.push("c", 2);
	list.push("b", 3);

	let mut iter = list.iter();
	assert_eq!(iter.next(), Some(&("b", 3)));
	assert_eq!(iter.next(), Some(&("c", 2)));
	assert_eq!(iter.next(), Some(&("a", 1)));
    }

    #[test]
    fn iter_mut() {
	let mut list = List::new();
	list.push("a", 1);
	list.push("c", 2);
	list.push("b", 3);

	let mut iter = list.iter_mut();
	assert_eq!(iter.next(), Some(&mut ("b", 3)));
	assert_eq!(iter.next(), Some(&mut ("c", 2)));
	assert_eq!(iter.next(), Some(&mut ("a", 1)));
    }

    #[test]
    fn update() {
	let mut list = List::new();
	list.push("a", 1);
	list.update("a", 2);

	assert_eq!(list.pop(), Some(("a", 2)));
    }
}
