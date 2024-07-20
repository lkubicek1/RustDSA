use std::fmt;
use std::marker::PhantomData;
use std::iter::FromIterator;
use crate::trees::heaps::heap_trait::Heap;

pub struct PriorityQueue<T, P, H>
where
    P: Ord + fmt::Debug,
    H: Heap<(P, T)>,
{
    heap: H,
    _marker: PhantomData<(T, P)>,
}

impl<T, P, H> PriorityQueue<T, P, H>
where
    P: Ord + fmt::Debug,
    H: Heap<(P, T)>,
{
    pub fn new(heap: H) -> Self {
        PriorityQueue {
            heap,
            _marker: PhantomData,
        }
    }

    pub fn push(&mut self, item: T, priority: P) {
        self.heap.push((priority, item));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.heap.pop().map(|(_, item)| item)
    }

    pub fn peek(&self) -> Option<&T> {
        self.heap.peek().map(|(_, item)| item)
    }

    pub fn peek_with_priority(&self) -> Option<(&P, &T)> {
        self.heap.peek().map(|(priority, item)| (priority, item))
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn extend<I: IntoIterator<Item = (P, T)>>(&mut self, iter: I) {
        for (priority, item) in iter {
            self.push(item, priority);
        }
    }

    pub fn drain(&mut self) -> impl Iterator<Item = T> + '_ {
        std::iter::from_fn(move || self.pop())
    }

    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&P, &T) -> bool,
    {
        let mut retained = Vec::new();
        while let Some((priority, item)) = self.heap.pop() {
            if f(&priority, &item) {
                retained.push((priority, item));
            }
        }
        for (priority, item) in retained {
            self.heap.push((priority, item));
        }
    }
}

impl<T, P, H> FromIterator<(P, T)> for PriorityQueue<T, P, H>
where
    P: Ord + fmt::Debug,
    H: Heap<(P, T)> + Default,
{
    fn from_iter<I: IntoIterator<Item = (P, T)>>(iter: I) -> Self {
        let mut queue = PriorityQueue::new(H::new());
        queue.extend(iter);
        queue
    }
}

impl<T, P, H> IntoIterator for PriorityQueue<T, P, H>
where
    P: Ord + fmt::Debug,
    H: Heap<(P, T)>,
{
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut vec = Vec::new();
        let mut queue = self;
        while let Some(item) = queue.pop() {
            vec.push(item);
        }
        vec.into_iter()
    }
}

impl<T, P, H> fmt::Display for PriorityQueue<T, P, H>
where
    T: fmt::Debug,
    P: Ord + fmt::Debug,
    H: Heap<(P, T)> + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PriorityQueue {}", self.heap)
    }
}