///////////////////////////////////////////////////////////////////////////////
//        Search in an unordered symbol table using sequential search        //
///////////////////////////////////////////////////////////////////////////////

use crate::data_structures::linked_list_kv::List;

pub struct ST<U, T> {
    pub list: List<U, T>,
    pub size: usize,
}

impl<U: std::cmp::PartialEq + Copy, T: Ord + Copy> ST<U, T> {
    pub fn new() -> Self {
        ST {
            list: List::new(),
            size: 0,
        }
    }

    pub fn get(&self, key: U) -> Option<T> {
        let iter = self.list.iter();

        for item in iter {
            if item.0 == key {
                return Some(item.1.clone());
            }
        }

        None
    }

    pub fn put(&mut self, key: U, value: T) {
        let hit = self.get(key);

        if hit.is_some() {
            self.list.update(key, value);
        } else {
            self.list.push(key, value);
            self.size += 1;
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn keys(&mut self) -> Vec<U> {
        let mut keys = Vec::new();
        let iter = self.list.iter();

        for item in iter {
            keys.push(item.0);
        }

        keys
    }

    fn delete(&mut self, key: U) {
        todo!("Delete function")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let mut st = ST::new();
        st.put("a", 1);
        st.put("b", 2);

        assert_eq!(st.get("a"), Some(1));
        assert_eq!(st.get("e"), None);

        st.put("a", 3);
        assert_eq!(st.get("a"), Some(3));

        st.put("c", 6);
        assert_eq!(st.get("c"), Some(6));

        assert_eq!(st.size(), 3);
        assert_eq!(st.keys(), vec!["c", "b", "a"]);
    }
}
