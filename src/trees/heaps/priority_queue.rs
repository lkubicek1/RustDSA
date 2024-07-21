use std::cmp::Ordering;
use std::fmt;
use std::marker::PhantomData;
use crate::trees::heaps::max_heap::MaxHeapType;
use crate::trees::heaps::min_heap::MinHeapType;
use super::heap::{GenericHeap, HeapType, Heap};

#[derive(Debug)]
struct PriorityItem<P, T> {
    priority: P,
    item: T,
    index: usize,
}

impl<P: Ord, T> PartialOrd for PriorityItem<P, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<P: Ord, T> Ord for PriorityItem<P, T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
            .then_with(|| self.index.cmp(&other.index))
    }
}

impl<P: PartialEq, T> PartialEq for PriorityItem<P, T> {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority && self.index == other.index
    }
}

impl<P: Eq, T> Eq for PriorityItem<P, T> {}

pub struct PriorityQueue<T, P, H>
where
    P: Ord + fmt::Debug,
    T: fmt::Debug,
    H: HeapType,
{
    heap: GenericHeap<PriorityItem<P, T>, H>,
    counter: usize,
    _marker: PhantomData<(T, P)>,
}

impl<T, P, H> PriorityQueue<T, P, H>
where
    P: Ord + fmt::Debug,
    T: fmt::Debug,
    H: HeapType,
{
    pub fn new() -> Self {
        Self {
            heap: GenericHeap::new(),
            counter: 0,
            _marker: PhantomData,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            heap: GenericHeap::with_capacity(capacity),
            counter: 0,
            _marker: PhantomData,
        }
    }

    pub fn push(&mut self, item: T, priority: P) {
        let priority_item = PriorityItem {
            priority,
            item,
            index: self.counter,
        };
        self.heap.push(priority_item);
        self.counter += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.heap.pop().map(|priority_item| priority_item.item)
    }

    pub fn pop_with_priority(&mut self) -> Option<(P, T)> {
        self.heap.pop().map(|priority_item| (priority_item.priority, priority_item.item))
    }

    pub fn peek(&self) -> Option<&T> {
        self.heap.peek().map(|priority_item| &priority_item.item)
    }

    pub fn peek_with_priority(&self) -> Option<(&P, &T)> {
        self.heap.peek().map(|priority_item| (&priority_item.priority, &priority_item.item))
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn clear(&mut self) {
        self.heap.clear();
        self.counter = 0;
    }

    pub fn drain(&mut self) -> impl Iterator<Item = T> + '_ {
        std::iter::from_fn(move || self.pop())
    }

    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&P, &T) -> bool,
    {
        self.heap.retain(|priority_item| f(&priority_item.priority, &priority_item.item));
    }

    pub fn into_sorted_vec(self) -> Vec<(P, T)> {
        let mut vec: Vec<_> = self.heap
            .into_vec()
            .into_iter()
            .map(|priority_item| (priority_item.priority, priority_item.item))
            .collect();
        vec.sort_by(|(a_prio, _), (b_prio, _)| H::comparison_fn()(a_prio, b_prio));
        vec
    }
}

impl<T, P, H> fmt::Display for PriorityQueue<T, P, H>
where
    P: Ord + fmt::Debug,
    T: fmt::Debug,
    H: HeapType,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PriorityQueue(")?;
        for (i, priority_item) in self.heap.heap.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "({:?}, {:?})", priority_item.priority, priority_item.item)?;
        }
        write!(f, ")")
    }
}

impl<T, P, H> FromIterator<(P, T)> for PriorityQueue<T, P, H>
where
    P: Ord + fmt::Debug,
    T: fmt::Debug,
    H: HeapType,
{
    fn from_iter<I: IntoIterator<Item = (P, T)>>(iter: I) -> Self {
        let mut pq = Self::new();
        for (priority, item) in iter {
            pq.push(item, priority);
        }
        pq
    }
}

impl<T, P, H> Extend<(P, T)> for PriorityQueue<T, P, H>
where
    P: Ord + fmt::Debug,
    T: fmt::Debug,
    H: HeapType,
{
    fn extend<I: IntoIterator<Item = (P, T)>>(&mut self, iter: I) {
        for (priority, item) in iter {
            self.push(item, priority);
        }
    }
}

pub type MaxPriorityQueue<T, P> = PriorityQueue<T, P, MaxHeapType>;
pub type MinPriorityQueue<T, P> = PriorityQueue<T, P, MinHeapType>;
