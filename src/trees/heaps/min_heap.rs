use std::cmp::Ordering;
use crate::trees::heaps::heap::{GenericHeap, HeapType};

#[derive(Debug)]
pub struct MinHeapType;

impl HeapType for MinHeapType {
    fn type_name() -> &'static str {
        "MinHeap"
    }

    fn comparison_fn<T: Ord>() -> fn(&T, &T) -> Ordering {
        |a, b| a.cmp(b)
    }

    fn arity() -> usize {
       2
    }
}

pub type MinHeap<T> = GenericHeap<T, MinHeapType>;
