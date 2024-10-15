pub trait UnionFind<T> {
    fn union(&mut self, p: T, q: T);
    fn find(&self, p: T) -> usize;
    fn connected(&self, p: T, q: T) -> bool;
    fn count(&self) -> usize;
}
