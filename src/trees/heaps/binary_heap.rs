use std::fmt;
use std::cmp::Ordering;
use crate::trees::heaps::heap_trait::Heap;

pub struct BinaryHeap<T, F>
where
    T: Ord + fmt::Debug,
    F: Fn(&T, &T) -> Ordering,
{
    pub(crate) heap: Vec<T>,
    compare: F,
}

impl<T, F> BinaryHeap<T, F>
where
    T: Ord + fmt::Debug,
    F: Fn(&T, &T) -> Ordering,
{
    pub fn new(compare: F) -> Self {
        BinaryHeap {
            heap: Vec::new(),
            compare,
        }
    }

    fn heapify_up(&mut self, mut index: usize) {
        while index > 0 {
            let parent = (index - 1) / 2;
            if (self.compare)(&self.heap[index], &self.heap[parent]) == Ordering::Less {
                self.heap.swap(index, parent);
                index = parent;
            } else {
                break;
            }
        }
    }

    fn heapify_down(&mut self, mut index: usize) {
        let len = self.heap.len();
        loop {
            let left_child = 2 * index + 1;
            let right_child = 2 * index + 2;
            let mut extreme = index;

            if left_child < len && (self.compare)(&self.heap[left_child], &self.heap[extreme]) == Ordering::Less {
                extreme = left_child;
            }

            if right_child < len && (self.compare)(&self.heap[right_child], &self.heap[extreme]) == Ordering::Less {
                extreme = right_child;
            }

            if extreme != index {
                self.heap.swap(index, extreme);
                index = extreme;
            } else {
                break;
            }
        }
    }
}

impl<T, F> Heap<T> for BinaryHeap<T, F>
where
    T: Ord + fmt::Debug,
    F: Fn(&T, &T) -> Ordering,
{
    fn new() -> Self where Self: Sized {
        unreachable!("Use BinaryHeap::new(compare) instead")
    }

    fn push(&mut self, item: T) {
        self.heap.push(item);
        self.heapify_up(self.heap.len() - 1);
    }

    fn pop(&mut self) -> Option<T> {
        if self.heap.is_empty() {
            None
        } else {
            let last = self.heap.pop().unwrap();
            if !self.heap.is_empty() {
                let first = std::mem::replace(&mut self.heap[0], last);
                self.heapify_down(0);
                Some(first)
            } else {
                Some(last)
            }
        }
    }

    fn peek(&self) -> Option<&T> {
        self.heap.first()
    }

    fn len(&self) -> usize {
        self.heap.len()
    }

    fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
}

impl<T, F> fmt::Display for BinaryHeap<T, F>
where
    T: Ord + fmt::Debug,
    F: Fn(&T, &T) -> Ordering,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BinaryHeap [")?;
        for (i, item) in self.heap.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:?}", item)?;
        }
        write!(f, "]")
    }
}