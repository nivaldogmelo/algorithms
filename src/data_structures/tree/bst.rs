use std::cmp::Ordering;

type Node<K, V> = Box<BST<K, V>>;

struct BST<K, V> {
    key: K,
    value: Option<V>,
    left: Option<Node<K, V>>,
    right: Option<Node<K, V>>,
    count: usize,
}

impl<K, V> BST<K, V>
where
    K: std::cmp::PartialOrd,
{
    fn get(&self, key: K) -> Option<&V> {
	match self.search(key) {
	    Some(node) => node.value.as_ref(),
	    None => None,
	}
    }

    fn insert(&mut self, key: K, value: V) {
	match self.search(key) {
	    Some(node) => {
		// let mut nvalue = node.value.as_ref();
		// nvalue = Some(&value)
	    }
	    None => todo!(),
	}
	todo!()
    }

    fn size(&self) -> usize {
	todo!()
    }

    fn search(&self, key: K) -> Option<&BST<K, V>> {
	match key.partial_cmp(&self.key) {
	    Some(Ordering::Equal) => Some(self),
	    Some(Ordering::Less) => {
		if self.left.is_none() {
		    None
		} else {
		    let left = self.left.as_ref().unwrap();
		    return left.search(key);
		}
	    }
	    Some(Ordering::Greater) => {
		if self.right.is_none() {
		    None
		} else {
		    let right = self.right.as_ref().unwrap();
		    return right.search(key);
		}
	    }
	    None => None,
	}
    }
}
