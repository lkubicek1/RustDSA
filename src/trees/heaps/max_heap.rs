use std::cmp::Ordering;
use std::fmt;
use std::iter::FromIterator;
use crate::trees::heaps::binary_heap::BinaryHeap;
use crate::trees::heaps::heap_trait::Heap;

pub struct MaxHeap<T: Ord + fmt::Debug>(BinaryHeap<T, fn(&T, &T) -> Ordering>);

impl<T: Ord + fmt::Debug> MaxHeap<T> {
    pub fn new() -> Self {
        MaxHeap(BinaryHeap::new(|a, b| b.cmp(a)))
    }

    pub fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push(item);
        }
    }

    pub fn drain(&mut self) -> impl Iterator<Item = T> + '_ {
        std::iter::from_fn(move || self.pop())
    }

    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> bool,
    {
        let mut new_heap = MaxHeap::new();
        while let Some(item) = self.pop() {
            if f(&item) {
                new_heap.push(item);
            }
        }
        *self = new_heap;
    }
}

impl<T: Ord + fmt::Debug> Heap<T> for MaxHeap<T> {
    fn new() -> Self {
        Self::new()
    }

    fn push(&mut self, item: T) {
        self.0.push(item)
    }

    fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.0.peek()
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<T: Ord + fmt::Debug> FromIterator<T> for MaxHeap<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut heap = MaxHeap::new();
        heap.extend(iter);
        heap
    }
}

impl<T: Ord + fmt::Debug> IntoIterator for MaxHeap<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut vec = Vec::new();
        let mut heap = self;
        while let Some(item) = heap.pop() {
            vec.push(item);
        }
        vec.into_iter()
    }
}

impl<T: Ord + fmt::Debug> From<MaxHeap<T>> for Vec<T> {
    fn from(heap: MaxHeap<T>) -> Self {
        heap.into_iter().collect()
    }
}

impl<T: Ord + fmt::Debug> fmt::Display for MaxHeap<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MaxHeap [")?;
        let mut first = true;
        for item in self.0.heap.iter() {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "{:?}", item)?;
            first = false;
        }
        write!(f, "]")
    }
}