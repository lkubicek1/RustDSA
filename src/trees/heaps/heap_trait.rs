pub trait Heap<T> {
    fn new() -> Self where Self: Sized;
    fn push(&mut self, item: T);
    fn pop(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}