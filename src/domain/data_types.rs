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

pub trait SymbolTable<K, V> {
    fn put(&mut self, key: K, value: V);
    fn get(&mut self, key: K) -> Option<&V>;
    fn delete(&mut self, key: K);
    fn contains(&self, key: K) -> bool;
    fn is_empty(&self) -> bool;
    fn size(&self) -> usize;
}
