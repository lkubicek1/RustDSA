use super::heap::{Heap, HeapType, HeapCreate};
use std::cmp::Ordering;
use std::fmt;

pub struct MaxHeapType;

impl HeapType for MaxHeapType {
    fn type_name() -> &'static str {
        "MaxHeap"
    }
}

impl<T: Ord + fmt::Debug> HeapCreate<T> for MaxHeapType {
    fn create_heap() -> Heap<T, fn(&T, &T) -> Ordering, Self> {
        Heap::with_capacity(2, Self::comparison_fn(), 0)
    }

    fn comparison_fn() -> fn(&T, &T) -> Ordering {
        |a, b| b.cmp(a)
    }
}

pub type MaxHeap<T> = Heap<T, fn(&T, &T) -> Ordering, MaxHeapType>;

impl<T: Ord + fmt::Debug> MaxHeap<T> {
    pub fn new() -> Self {
        MaxHeapType::create_heap()
    }
}