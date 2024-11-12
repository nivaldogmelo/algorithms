use crate::{data_structures::LinkedList, domain::SymbolTable};

struct Node<K, V> {
    key: K,
    value: Option<V>,
}

impl<K, V> Node<K, V> {
    pub fn new(key: K, value: Option<V>) -> Self {
        Self { key, value }
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
        let key_exists = self.contains(&key);

        if key_exists {
            for node in self.itens.iter_mut() {
                if node.key == key {
                    node.value = value;
                    break;
                }
            }
        } else {
            let item = Node::new(key, value);
            self.itens.insert_at_beginning(item);
        }
    }

    fn get(&self, key: &K) -> Option<&V> {
        for node in self.itens.iter() {
            if node.key == *key {
                return node.value.as_ref();
            }
        }

        None
    }
    fn delete(&mut self, key: K) {
        self.put(key, None);
    }

    fn contains(&self, key: &K) -> bool {
        self.get(&key).is_some()
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
