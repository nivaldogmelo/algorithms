///////////////////////////////////////////////////////////////////////////////
//           Search in an ordered symbol table using binary search           //
///////////////////////////////////////////////////////////////////////////////

use rand::seq::index;

pub struct OrderedST<U, T> {
    pub key: Vec<U>,
    pub value: Vec<T>,
    pub size: usize,
}

impl<U: Ord + Copy, T: Copy> OrderedST<U, T> {
    pub fn new() -> Self {
        Self {
            key: Vec::new(),
            value: Vec::new(),
            size: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&mut self) -> bool {
        if self.size > 0 {
            false
        } else {
            true
        }
    }

    fn rank(&mut self, key: U) -> usize {
        let mut lower = 0;
        let mut higher = self.size;

        while lower < higher {
            let index = (higher + lower) / 2;
            if self.key[index] < key {
                higher = index;
            } else if self.key[index] > key {
                lower = index + 1;
            } else {
                return index;
            }
        }

        return lower;
    }

    pub fn get(&mut self, key: U) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let index = self.rank(key);

        if index < self.size && self.key[index] == key {
            return Some(self.value[index].clone());
        } else {
            None
        }
    }
    pub fn put(&mut self, key: U, value: T) {
        let index = self.rank(key);
        if index < self.size && self.key[index] == key {
            self.value[index] = value;
            return;
        }

        self.key.push(key);
        self.value.push(value);

        for i in (index..self.size).rev() {
            self.key[i + 1] = self.key[i];
            self.value[i + 1] = self.value[i];
        }
        self.size += 1;

        self.key[index] = key;
        self.value[index] = value;
    }

    pub fn delete(&mut self, key: U) {
        if self.is_empty() {
            return;
        }

        let index = self.rank(key);

        if index < self.size && self.key[index] == key {
            for i in (index..(self.size - 1)) {
                self.key.swap(i + 1, i);
                self.value.swap(i + 1, i);
            }

            self.key.pop();
            self.value.pop();
            self.size -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let mut st = OrderedST::new();
        assert_eq!(st.size(), 0);

        st.put("t", 3);
        st.put("v", 8);
        st.put("d", 11);

        assert_eq!(st.size(), 3);
        assert_eq!(st.get("v"), Some(8));

        st.put("d", 8);
        assert_eq!(st.get("d"), Some(8));

        st.delete("d");
        assert_eq!(st.get("d"), None);
        assert_eq!(st.size(), 2);
    }
}
