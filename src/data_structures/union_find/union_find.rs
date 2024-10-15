use std::collections::HashMap;

use crate::domain::UnionFind;

struct UF<T: std::hash::Hash + Eq> {
    size: usize,
    point: HashMap<T, usize>,
}

impl<T: std::hash::Hash + Eq> UF<T> {
    fn new() -> Self {
        let point = HashMap::new();
        Self { size: 0, point }
    }

    fn insert(&mut self, p: T) {
        self.point.insert(p, self.size + 1);
        self.size += 1;
    }
}

impl<T: std::hash::Hash + Eq> UnionFind<T> for UF<T> {
    fn union(&mut self, p: T, q: T) {
        let class_p = self.find(p);
        let class_q = self.find(q);

        if class_p != class_q {
            self.point.iter_mut().for_each(|(_, class)| {
                if *class == class_q {
                    *class = class_p;
                }
            });

            self.size -= 1;
        }
    }

    fn find(&self, p: T) -> usize {
        match self.point.get(&p) {
            Some(i) => *i,
            None => 0,
        }
    }

    fn connected(&self, p: T, q: T) -> bool {
        self.find(p) == self.find(q)
    }

    fn count(&self) -> usize {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union() {
        let mut uf = UF::<u32>::new();

        uf.insert(2);
        uf.insert(3);
        uf.insert(4);
        assert_eq!(uf.count(), 3);

        uf.union(2, 3);
        uf.union(2, 4);

        assert!(uf.connected(2, 3));
        assert!(!uf.connected(2, 5));
    }
}
