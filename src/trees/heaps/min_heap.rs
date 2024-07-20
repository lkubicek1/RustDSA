use super::heap::{Heap, HeapType, HeapCreate};
use std::cmp::Ordering;
use std::fmt;

pub struct MinHeapType;

impl HeapType for MinHeapType {
    fn type_name() -> &'static str {
        "MinHeap"
    }
}

impl<T: Ord + fmt::Debug> HeapCreate<T> for MinHeapType {
    fn create_heap() -> Heap<T, fn(&T, &T) -> Ordering, Self> {
        Heap::with_capacity(2, Self::comparison_fn(), 0)
    }

    fn comparison_fn() -> fn(&T, &T) -> Ordering {
        |a, b| a.cmp(b)
    }
}

pub type MinHeap<T> = Heap<T, fn(&T, &T) -> Ordering, MinHeapType>;

impl<T: Ord + fmt::Debug> MinHeap<T> {
    pub fn new() -> Self {
        MinHeapType::create_heap()
    }
}
