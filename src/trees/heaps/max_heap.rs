use std::cmp::Ordering;
use crate::trees::heaps::heap::{GenericHeap, HeapType};

#[derive(Debug)]
pub struct MaxHeapType;

impl HeapType for MaxHeapType {
    fn type_name() -> &'static str {
        "MaxHeap"
    }

    fn comparison_fn<T: Ord>() -> fn(&T, &T) -> Ordering {
        |a, b| b.cmp(a)
    }
}

pub type MaxHeap<T> = GenericHeap<T, MaxHeapType>;
