type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    item: T,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(item: T) -> Node<T> {
        Node { item, next: None }
    }

    fn add_next(&mut self, next: Link<T>) {
        self.next = next;
    }
}

pub struct LinkedList<T> {
    head: Link<T>,
    size: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            size: 0,
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
                Some(node.item)
            }
            None => None,
        }
    }
}

impl<T> Iterator for LinkedList<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let cur_head = self.head.take();

        match cur_head {
            Some(node) => {
                self.head = node.next;
                Some(node.item)
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length() {
        let mut ll: LinkedList<i32> = LinkedList::<i32>::new();
        assert_eq!(ll.length(), 0);

        ll.insert_at_beginning(1);
        assert_eq!(ll.length(), 1);
    }

    #[test]
    fn test_insert_at_beginning() {
        let mut ll: LinkedList<i32> = LinkedList::<i32>::new();

        ll.insert_at_beginning(1);
        ll.insert_at_beginning(2);
        ll.insert_at_beginning(3);

        assert_eq!(ll.collect::<Vec<i32>>(), vec![3, 2, 1]);
    }

    #[test]
    fn test_remove_from_beginning() {
        let mut ll: LinkedList<i32> = LinkedList::<i32>::new();

        ll.insert_at_beginning(1);
        ll.insert_at_beginning(2);
        ll.insert_at_beginning(3);

        let item = ll.remove_from_beginning();
        assert_eq!(item, Some(3));

        let item = ll.remove_from_beginning();
        assert_eq!(item, Some(2));

        let item = ll.remove_from_beginning();
        assert_eq!(item, Some(1));
    }
}
