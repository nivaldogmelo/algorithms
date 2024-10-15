pub trait Bag<T> {
    fn add(&self, item: T);
    fn is_empty(&self) -> bool;
    fn size(&self) -> usize;
}

pub trait Queue<T> {
    fn enqueue(&self, item: T);
    fn dequeue(&self) -> T;
    fn is_empty(&self) -> bool;
    fn size(&self) -> usize;
}

pub trait Stack<T> {
    fn push(&self, item: T);
    fn pop(&self) -> T;
    fn is_empty(&self) -> bool;
    fn size(&self) -> usize;
}
