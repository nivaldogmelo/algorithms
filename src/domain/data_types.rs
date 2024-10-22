pub trait UnionFind<T> {
    fn union(&mut self, p: T, q: T);
    fn find(&self, p: T) -> usize;
    fn connected(&self, p: T, q: T) -> bool;
    fn count(&self) -> usize;
}

pub trait PriorityQueue<T> {
    fn insert(&mut self, key: T);
    fn max(&self) -> &T;
    fn del_max(&mut self) -> Option<T>;
    fn is_empty(&self) -> bool;
    fn size(&self) -> usize;
}
