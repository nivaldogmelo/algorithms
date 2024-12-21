type Link<T> = Box<Node<T>>;

struct Node<T> {
    item: T,
    next: Option<Link<T>>,
}

impl<T> Node<T> {
    fn new(item: T) -> Node<T> {
        Node { item, next: None }
    }

    fn add_next(&mut self, next: Option<Link<T>>) {
        self.next = next;
    }
}

pub struct LinkedList<T> {
    head: Option<Link<T>>,
    size: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }

    pub fn iter(&self) -> LinkedListIter<T> {
        LinkedListIter {
            current: self.head.as_deref(),
        }
    }

    pub fn iter_mut(&mut self) -> LinkedListIterMut<T> {
        LinkedListIterMut {
            current: self.head.as_deref_mut(),
        }
    }

    pub fn length(&self) -> usize {
        self.size
    }

    pub fn insert_at_beginning(&mut self, item: T) {
        let cur_head = self.head.take();

        let mut node = Node::new(item);
        node.add_next(cur_head);

        self.head = Some(Box::new(node));
        self.size += 1;
    }

    pub fn remove_from_beginning(&mut self) -> Option<T> {
        let cur_head = self.head.take();

        match cur_head {
            Some(node) => {
                self.head = node.next;
                self.size -= 1;
                Some(node.item)
            }
            None => None,
        }
    }
}

pub struct LinkedListIter<'a, T> {
    current: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for LinkedListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Some(node) => {
                self.current = node.next.as_deref();
                Some(&node.item)
            }
            None => None,
        }
    }
}

pub struct LinkedListIterMut<'a, T> {
    current: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for LinkedListIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|node| {
            self.current = node.next.as_deref_mut();
            &mut node.item
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use quickcheck::QuickCheck;

    fn prop_number(data: Vec<i32>) -> Result<(), String> {
        let mut linked_list = LinkedList::new();
        let size = data.len();
        let mut test_data = data.clone();

        (0..data.len()).for_each(|i| {
            linked_list.insert_at_beginning(data[i]);
        });

        if linked_list.length() != size {
            return Err(format!(
                "Size mismatch after insert: expected {}, got {}",
                size,
                linked_list.length()
            ));
        }

        for value in linked_list.iter() {
            if !data.contains(value) {
                return Err(format!(
                    "Value mismatch on insert: value {} not found",
                    *value,
                ));
            }
        }

        for _i in 0..linked_list.length() {
            let value = linked_list.remove_from_beginning();

            if value != test_data.pop() {
                return Err(format!(
                    "value mismatch on remove: expected {:?}, got {:?}",
                    test_data.pop(),
                    Some(value),
                ));
            }
        }

        if linked_list.length() != test_data.len() {
            return Err(format!(
                "Size mismatch after remove: expected {}, got {}",
                test_data.len(),
                linked_list.length()
            ));
        }

        Ok(())
    }

    fn prop_string(data: Vec<String>) -> Result<(), String> {
        let mut linked_list = LinkedList::new();
        let size = data.len();
        let mut test_data = data.clone();

        (0..data.len()).for_each(|i| {
            linked_list.insert_at_beginning(data[i].clone());
        });

        if linked_list.length() != size {
            return Err(format!(
                "Size mismatch after insert: expected {}, got {}",
                size,
                linked_list.length()
            ));
        }

        for value in linked_list.iter() {
            if !data.contains(value) {
                return Err(format!(
                    "Value mismatch on insert: value {} not found",
                    *value,
                ));
            }
        }

        for _i in 0..linked_list.length() {
            let value = linked_list.remove_from_beginning();

            if value != test_data.pop() {
                return Err(format!(
                    "value mismatch on remove: expected {:?}, got {:?}",
                    test_data.pop(),
                    Some(value),
                ));
            }
        }

        if linked_list.length() != test_data.len() {
            return Err(format!(
                "Size mismatch after remove: expected {}, got {}",
                test_data.len(),
                linked_list.length()
            ));
        }

        Ok(())
    }

    #[test]
    fn quickcheck_get() {
        QuickCheck::new()
            .tests(1000)
            .quickcheck(prop_number as fn(Vec<i32>) -> Result<(), String>);
        QuickCheck::new()
            .tests(1000)
            .quickcheck(prop_string as fn(Vec<String>) -> Result<(), String>);
    }
}
