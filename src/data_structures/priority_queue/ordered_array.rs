use crate::domain::PriorityQueue;

pub struct PQ<T> {
    itens: Vec<T>,
    size: usize,
}

impl<T> PQ<T> {
    pub fn new() -> Self {
        Self {
            itens: vec![],
            size: 0,
        }
    }
}

impl<T> PQ<T>
where
    T: Default,
{
    pub fn new_from_array(a: &mut [T]) -> Self {
        let mut itens = vec![];
        let size = a.len();

        for i in 0..a.len() - 1 {
            let value = std::mem::take(&mut a[i]);

            itens.push(value);
        }

        Self { itens, size }
    }
}

impl<T: std::cmp::PartialOrd> PriorityQueue<T> for PQ<T> {
    fn insert(&mut self, key: T) {
        self.itens.push(key);
        self.size += 1;

        if self.size > 1 {
            for i in 1..self.itens.len() - 1 {
                if self.itens[i] < self.itens[i - 1] {
                    self.itens.swap(i, i - 1);
                }
            }
        }
    }

    fn max(&self) -> &T {
        &self.itens[self.size - 1]
    }

    fn del_max(&mut self) -> Option<T> {
        self.size -= 1;
        self.itens.pop()
    }

    fn is_empty(&self) -> bool {
        if self.size > 0 {
            false
        } else {
            true
        }
    }

    fn size(&self) -> usize {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_from_array() {
        let mut array = [3, 6, 4, 2, 8];
        let array_final = array;
        let size = array.len();

        let pq = PQ::new_from_array(&mut array);

        assert_eq!(pq.size(), size);

        (0..size - 1).for_each(|i| assert_eq!(array_final[i], pq.itens[i]));
    }

    #[test]
    fn test_insert() {
        let mut pq: PQ<i32> = PQ::<i32>::new();

        pq.insert(1);

        assert_eq!(pq.itens[0], 1);
    }

    #[test]
    fn test_max() {
        let mut pq: PQ<i32> = PQ::<i32>::new();

        pq.insert(1);
        pq.insert(3);

        assert_eq!(pq.max(), &3);
        assert_eq!(pq.size(), 2);
    }

    #[test]
    fn test_del_max() {
        let mut pq: PQ<i32> = PQ::<i32>::new();

        pq.insert(1);
        pq.insert(3);

        assert_eq!(pq.del_max(), Some(3));
        assert_eq!(pq.size(), 1);
    }

    #[test]
    fn test_is_empty() {
        let mut pq: PQ<i32> = PQ::<i32>::new();
        assert!(pq.is_empty());

        pq.insert(1);
        assert!(!pq.is_empty());
    }

    #[test]
    fn test_size() {
        let mut pq: PQ<i32> = PQ::<i32>::new();
        assert_eq!(pq.size(), 0);

        pq.insert(1);
        assert_eq!(pq.size(), 1);

        pq.insert(1);
        pq.insert(1);
        pq.del_max();
        assert_eq!(pq.size(), 2);
    }
}
