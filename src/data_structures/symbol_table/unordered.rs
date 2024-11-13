use std::cmp::Ordering;

use crate::{data_structures::LinkedList, domain::SymbolTable, search::sequential_search};

struct Node<K, V> {
    key: K,
    value: Option<V>,
}

impl<K, V> Node<K, V> {
    pub fn new(key: K, value: Option<V>) -> Self {
	Self { key, value }
    }
}

impl<K: PartialOrd, V> PartialOrd for Node<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
	self.key.partial_cmp(&other.key)
    }
}

impl<K: PartialOrd, V> PartialEq for Node<K, V> {
    fn eq(&self, other: &Self) -> bool {
	self.key == other.key
    }
}

pub struct UnorderedST<K, V> {
    itens: LinkedList<Node<K, V>>,
}

impl<K, V> UnorderedST<K, V> {
    pub fn new() -> Self {
	let itens = LinkedList::new();
	Self { itens }
    }
}

impl<K, V> SymbolTable<K, V> for UnorderedST<K, V>
where
    K: std::cmp::PartialEq,
{
    fn put(&mut self, key: K, value: Option<V>) {
	let mut iter: Vec<_> = self.itens.iter_mut().collect();
	let keys: Vec<_> = iter.iter().map(|node| &node.key).collect();

	match sequential_search(&keys, &key) {
	    Some(i) => {
		let aux = &mut iter[i];
		aux.value = value;
	    }
	    None => {
		let item = Node::new(key, value);
		self.itens.insert_at_beginning(item);
	    }
	}
    }

    fn get(&self, key: &K) -> Option<&V> {
	let iter: Vec<&Node<K, V>> = self.itens.iter().collect();
	let keys: Vec<_> = iter.iter().map(|node| &node.key).collect();

	match sequential_search(&keys, key) {
	    Some(i) => {
		let aux = iter[i];
		aux.value.as_ref()
	    }
	    None => None,
	}

	// for node in self.itens.iter() {
	//     if node.key == *key {
	//	return node.value.as_ref();
	//     }
	// }
    }
    fn delete(&mut self, key: K) {
	self.put(key, None);
    }

    fn contains(&self, key: &K) -> bool {
	self.get(key).is_some()
    }

    fn is_empty(&self) -> bool {
	self.itens.length() < 1
    }

    fn size(&self) -> usize {
	self.itens.length()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    use quickcheck::QuickCheck;

    #[test]
    fn test_put_same_key() {
	let mut unordered_st = UnorderedST::new();
	unordered_st.put(32, Some(5));
	unordered_st.put(32, Some(2));

	assert_eq!(unordered_st.size(), 1);
    }

    fn prop_number_number(data: HashMap<i32, i32>) -> Result<(), String> {
	let mut unordered_st = UnorderedST::new();
	let size = data.len();

	for (key, value) in data.iter() {
	    unordered_st.put(key, Some(value));
	}

	if unordered_st.size() != size {
	    return Err(format!(
		"Size mismatch after insert: expected {}, got {}",
		size,
		unordered_st.size()
	    ));
	}

	for item in unordered_st.itens.iter() {
	    if item.value != data.get(item.key) {
		return Err(format!(
		    "Value mismatch for key {} after insert: expected {:?}, got {:?}",
		    item.key,
		    data.get(item.key),
		    item.value
		));
	    }
	}

	for (key, value) in data.iter() {
	    match unordered_st.get(&key) {
		Some(st_value) => {
		    if st_value != &value {
			return Err(format!(
			    "Value mismatch for key {} in get: expected {}, got {}",
			    key, st_value, value
			));
		    }
		}
		None => {
		    return Err(format!(
			"Value mismatch for key {} in get: key does not exists",
			key
		    ));
		}
	    }
	}

	for (key, _) in data.iter() {
	    unordered_st.delete(key);
	    match unordered_st.get(&key) {
		Some(_) => {
		    return Err(format!(
			"Value mismatch for key {} in delete: Should return None",
			key
		    ))
		}
		None => {}
	    }
	}

	Ok(())
    }

    fn prop_string_number(data: HashMap<String, i32>) -> Result<(), String> {
	let mut unordered_st = UnorderedST::new();
	let size = data.len();

	for (key, value) in data.iter() {
	    unordered_st.put(key, Some(value));
	}

	if unordered_st.size() != size {
	    return Err(format!(
		"Size mismatch after insert: expected {}, got {}",
		size,
		unordered_st.size()
	    ));
	}

	for item in unordered_st.itens.iter() {
	    if item.value != data.get(item.key) {
		return Err(format!(
		    "Value mismatch for key {} after insert: expected {:?}, got {:?}",
		    item.key,
		    data.get(item.key),
		    item.value
		));
	    }
	}

	for (key, value) in data.iter() {
	    match unordered_st.get(&key) {
		Some(st_value) => {
		    if st_value != &value {
			return Err(format!(
			    "Value mismatch for key {} in get: expected {}, got {}",
			    key, st_value, value
			));
		    }
		}
		None => {
		    return Err(format!(
			"Value mismatch for key {} in get: key does not exists",
			key
		    ));
		}
	    }
	}

	for (key, _) in data.iter() {
	    unordered_st.delete(key);
	    match unordered_st.get(&key) {
		Some(_) => {
		    return Err(format!(
			"Value mismatch for key {} in delete: Should return None",
			key
		    ))
		}
		None => {}
	    }
	}

	Ok(())
    }

    #[test]
    fn quickcheck_unit_tests() {
	QuickCheck::new()
	    .tests(1000)
	    .quickcheck(prop_number_number as fn(HashMap<i32, i32>) -> Result<(), String>);
	QuickCheck::new()
	    .tests(1000)
	    .quickcheck(prop_string_number as fn(HashMap<String, i32>) -> Result<(), String>);
    }
}
