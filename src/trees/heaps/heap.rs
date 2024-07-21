use std::cmp::Ordering;
use std::fmt;
use std::marker::PhantomData;

pub trait Heap<T>
where
    T: Ord + fmt::Debug,
{
    fn new() -> Self
    where
        Self: Sized;
    fn with_capacity(capacity: usize) -> Self
    where
        Self: Sized;
    fn push(&mut self, item: T);
    fn pop(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn drain(&mut self) -> Box<dyn Iterator<Item = T> + '_>;
    fn retain<P>(&mut self, predicate: P)
    where
        P: FnMut(&T) -> bool;
    fn clear(&mut self);
}

pub trait HeapType: Sized + fmt::Debug {
    fn type_name() -> &'static str;
    fn comparison_fn<T: Ord>() -> fn(&T, &T) -> Ordering;
    fn arity() -> usize;
}

pub struct GenericHeap<T, H>
where
    T: Ord + fmt::Debug,
    H: HeapType,
{
    pub(crate) heap: Vec<T>,
    _marker: PhantomData<H>,
}

impl<T, H> GenericHeap<T, H>
where
    T: Ord + fmt::Debug,
    H: HeapType,
{
    fn heapify_up(&mut self, mut index: usize) {
        let compare = H::comparison_fn();
        let d = H::arity();
        while index > 0 {
            let parent = (index - 1) / d;
            if compare(&self.heap[index], &self.heap[parent]) == Ordering::Less {
                self.heap.swap(index, parent);
                index = parent;
            } else {
                break;
            }
        }
    }

    fn heapify_down(&mut self, mut index: usize) {
        let compare = H::comparison_fn();
        let d = H::arity();
        let len = self.heap.len();
        loop {
            let mut extreme = index;
            for i in 1..=d {
                let child = d * index + i;
                if child < len && compare(&self.heap[child], &self.heap[extreme]) == Ordering::Less {
                    extreme = child;
                }
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

impl<T, H> Heap<T> for GenericHeap<T, H>
where
    T: Ord + fmt::Debug,
    H: HeapType,
{
    fn new() -> Self {
        Self {
            heap: Vec::new(),
            _marker: PhantomData,
        }
    }

    fn with_capacity(capacity: usize) -> Self {
        Self {
            heap: Vec::with_capacity(capacity),
            _marker: PhantomData,
        }
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

    fn drain(&mut self) -> Box<dyn Iterator<Item = T> + '_> {
        Box::new(std::iter::from_fn(move || self.pop()))
    }

    fn retain<P>(&mut self, mut predicate: P)
    where
        P: FnMut(&T) -> bool,
    {
        let mut new_heap = Vec::new();
        while let Some(item) = self.pop() {
            if predicate(&item) {
                new_heap.push(item);
            }
        }
        self.heap = new_heap;
        for i in (0..self.heap.len()).rev() {
            self.heapify_down(i);
        }
    }

    fn clear(&mut self) {
        self.heap.clear();
    }
}

impl<T, H> fmt::Display for GenericHeap<T, H>
where
    T: Ord + fmt::Debug,
    H: HeapType,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}[", H::type_name())?;
        for (i, item) in self.heap.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:?}", item)?;
        }
        write!(f, "]")
    }
}

impl<T, H> From<GenericHeap<T, H>> for Vec<T>
where
    T: Ord + fmt::Debug,
    H: HeapType,
{
    fn from(mut heap: GenericHeap<T, H>) -> Self {
        let mut vec = Vec::with_capacity(heap.len());
        while let Some(item) = heap.pop() {
            vec.push(item);
        }
        vec
    }
}

impl<T, H> FromIterator<T> for GenericHeap<T, H>
where
    T: Ord + fmt::Debug,
    H: HeapType,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut heap = Self::new();
        for item in iter {
            heap.push(item);
        }
        heap
    }
}

impl<T, H> Extend<T> for GenericHeap<T, H>
where
    T: Ord + fmt::Debug,
    H: HeapType,
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push(item);
        }
    }
}

impl<T, H> GenericHeap<T, H>
where
    T: Ord + fmt::Debug,
    H: HeapType,
{
    pub fn into_vec(self) -> Vec<T> {
        self.heap
    }
}