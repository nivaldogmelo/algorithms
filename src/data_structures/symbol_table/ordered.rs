use crate::{data_structures::LinkedList, domain::SymbolTable, search::binary_search};

struct Node<K, V> {
    key: K,
    value: Option<V>,
}

impl<K, V> Node<K, V> {
    pub fn new(key: K, value: Option<V>) -> Self {
	Self { key, value }
    }
}

pub struct OrderedST<K, V>
where
    K: std::cmp::PartialOrd,
{
    itens: LinkedList<Node<K, V>>,
}

impl<K, V> OrderedST<K, V>
where
    K: std::cmp::PartialOrd,
{
    pub fn new() -> Self {
	let itens = LinkedList::new();
	Self { itens }
    }

    pub fn min(&self) -> Option<&K> {
	if self.itens.length() == 0 {
	    return None;
	}

	let iter: Vec<&Node<K, V>> = self.itens.iter().collect();
	let keys: Vec<_> = iter.iter().map(|node| &node.key).collect();

	Some(keys[0])
    }

    pub fn max(&self) -> Option<&K> {
	if self.itens.length() == 0 {
	    return None;
	}

	let iter: Vec<&Node<K, V>> = self.itens.iter().collect();
	let keys: Vec<_> = iter.iter().map(|node| &node.key).collect();

	Some(keys[keys.len() - 1])
    }

    pub fn floor(&self, key: &K) -> Option<&K> {
	let index = self.rank(key);

	let iter: Vec<&Node<K, V>> = self.itens.iter().collect();
	let keys: Vec<_> = iter.iter().map(|node| &node.key).collect();

	if keys[index] == key || index == 0 {
	    Some(keys[index])
	} else {
	    Some(keys[index - 1])
	}
    }

    pub fn ceiling(&self, key: &K) -> Option<&K> {
	let index = self.rank(key);

	let iter: Vec<&Node<K, V>> = self.itens.iter().collect();
	let keys: Vec<_> = iter.iter().map(|node| &node.key).collect();
	Some(keys[index])
    }

    pub fn rank(&self, key: &K) -> usize {
	let iter: Vec<&Node<K, V>> = self.itens.iter().collect();
	let keys: Vec<_> = iter.iter().map(|node| &node.key).collect();

	binary_search(keys.as_ref(), key)
    }

    pub fn select(&self, rank: usize) -> Option<&K> {
	let iter: Vec<&Node<K, V>> = self.itens.iter().collect();
	let keys: Vec<_> = iter.iter().map(|node| &node.key).collect();
	Some(keys[rank])
    }

    pub fn delete_min(&mut self) {
	self.itens.remove_from_beginning();
    }

    pub fn delete_max(&mut self) {
	if self.size() > 1 {
	    let mut itermut: Vec<&mut Node<K, V>> = self.itens.iter_mut().collect();
	    let lenght = itermut.len();

	    itermut.swap(0, lenght);

	    for i in 1..lenght - 2 {
		itermut.swap(i, lenght - 1);
	    }

	    self.itens.remove_from_beginning();
	} else if self.size() == 1 {
	    self.itens.remove_from_beginning();
	}
    }
}

impl<K, V> SymbolTable<K, V> for OrderedST<K, V>
where
    K: std::cmp::PartialOrd,
{
    fn put(&mut self, key: K, value: Option<V>) {
	let rank = self.rank(&key);

	let item = Node::new(key, value);
	self.itens.insert_at_beginning(item);

	let mut itermut: Vec<&mut Node<K, V>> = self.itens.iter_mut().collect();

	for i in 0..rank {
	    itermut.swap(i, i + 1);
	}
    }

    fn get(&self, key: &K) -> Option<&V> {
	let rank = self.rank(key);

	let iter: Vec<&Node<K, V>> = self.itens.iter().collect();

	if iter[rank].key == *key {
	    return iter[rank].value.as_ref();
	}

	None
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

    fn prop_number_number(data: HashMap<i32, i32>) -> Result<(), String> {
	let mut ordered_st = OrderedST::new();

	if ordered_st.size() != 0 {
	    return Err(format!(
		"Size mismatch after insert: expected {}, got {}",
		0,
		ordered_st.size()
	    ));
	}

	let size = data.len();
	for (key, value) in data.iter() {
	    ordered_st.put(*key, Some(*value));
	}

	if ordered_st.size() != size {
	    return Err(format!(
		"Size mismatch after insert: expected {}, got {}",
		size,
		ordered_st.size()
	    ));
	}

	if size > 0 {
	    let data_iter: Vec<(&i32, &i32)> = data.iter().collect();
	    let mut data_keys: Vec<&i32> = data_iter.iter().map(|kv| kv.0).collect();
	    data_keys.sort();
	    let data_min = data_keys[0];
	    let data_max = data_keys[size - 1];
	    if ordered_st.min() != Some(data_min) {
		return Err(format!(
		    "Min mismatch after insert: expected {:?}, got {:?}",
		    data_min,
		    ordered_st.min()
		));
	    }

	    if ordered_st.max() != Some(data_max) {
		return Err(format!(
		    "Max mismatch after insert: expected {:?}, got {:?}",
		    data_max,
		    ordered_st.min()
		));
	    }
	} else {
	    if ordered_st.min().is_some() {
		return Err(format!(
		    "Min mismatch after insert: expected None, got {:?}",
		    ordered_st.min()
		));
	    }

	    if ordered_st.max().is_some() {
		return Err(format!(
		    "Max mismatch after insert: expected None, got {:?}",
		    ordered_st.min()
		));
	    }
	}

	// for item in ordered_st.itens.iter() {
	//     if item.value != data.get(item.key) {
	//	return Err(format!(
	//	    "Value mismatch for key {} after insert: expected {:?}, got {:?}",
	//	    item.key,
	//	    data.get(item.key),
	//	    item.value
	//	));
	//     }
	// }

	// for (key, value) in data.iter() {
	//     match ordered_st.get(&key) {
	//	Some(st_value) => {
	//	    if st_value != &value {
	//		return Err(format!(
	//		    "Value mismatch for key {} in get: expected {}, got {}",
	//		    key, st_value, value
	//		));
	//	    }
	//	}
	//	None => {
	//	    return Err(format!(
	//		"Value mismatch for key {} in get: key does not exists",
	//		key
	//	    ));
	//	}
	//     }
	// }

	// for (key, _) in data.iter() {
	//     ordered_st.delete(key);
	//     match ordered_st.get(&key) {
	//	Some(_) => {
	//	    return Err(format!(
	//		"Value mismatch for key {} in delete: Should return None",
	//		key
	//	    ))
	//	}
	//	None => {}
	//     }
	// }

	Ok(())
    }

    #[test]
    fn quickcheck_get() {
	QuickCheck::new()
	    .tests(1000)
	    .quickcheck(prop_number_number as fn(HashMap<i32, i32>) -> Result<(), String>);
    }
}
