use std::cmp::Ordering;
use std::fmt;

pub trait HeapType: Sized {
    fn type_name() -> &'static str;
}

pub struct Heap<T, F, H>
where
    T: Ord + fmt::Debug,
    F: Fn(&T, &T) -> Ordering,
    H: HeapType,
{
    heap: Vec<T>,
    d: usize,
    compare: F,
    _marker: std::marker::PhantomData<H>,
}

impl<T, F, H> Heap<T, F, H>
where
    T: Ord + fmt::Debug,
    F: Fn(&T, &T) -> Ordering,
    H: HeapType,
{
    pub fn with_capacity(d: usize, compare: F, capacity: usize) -> Self {
        assert!(d >= 2, "d must be at least 2");
        Heap {
            heap: Vec::with_capacity(capacity),
            d,
            compare,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn push(&mut self, item: T) {
        self.heap.push(item);
        self.heapify_up(self.heap.len() - 1);
    }

    pub fn pop(&mut self) -> Option<T> {
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

    pub fn peek(&self) -> Option<&T> {
        self.heap.first()
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    fn parent(i: usize, d: usize) -> usize {
        (i - 1) / d
    }

    fn first_child(i: usize, d: usize) -> usize {
        i * d + 1
    }

    fn heapify_up(&mut self, mut index: usize) {
        while index > 0 {
            let parent = Self::parent(index, self.d);
            if (self.compare)(&self.heap[index], &self.heap[parent]) == Ordering::Less {
                self.heap.swap(index, parent);
                index = parent;
            } else {
                break;
            }
        }
    }

    fn heapify_down(&mut self, mut index: usize) {
        loop {
            let mut extreme = index;
            let first_child = Self::first_child(index, self.d);

            for child in first_child..std::cmp::min(first_child + self.d, self.heap.len()) {
                if (self.compare)(&self.heap[child], &self.heap[extreme]) == Ordering::Less {
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

    pub fn drain(&mut self) -> impl Iterator<Item = T> + '_ {
        std::iter::from_fn(move || self.pop())
    }

    pub fn retain<P>(&mut self, mut predicate: P)
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
        for i in (0..self.heap.len() / 2).rev() {
            self.heapify_down(i);
        }
    }
}

impl<T, F, H> From<Heap<T, F, H>> for Vec<T>
where
    T: Ord + fmt::Debug,
    F: Fn(&T, &T) -> Ordering,
    H: HeapType,
{
    fn from(mut heap: Heap<T, F, H>) -> Self {
        let mut vec = Vec::with_capacity(heap.len());
        while let Some(item) = heap.pop() {
            vec.push(item);
        }
        vec
    }
}

impl<T, H> FromIterator<T> for Heap<T, fn(&T, &T) -> Ordering, H>
where
    T: Ord + fmt::Debug,
    H: HeapCreate<T>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut heap = Self::with_capacity(2, H::comparison_fn(), 0);
        for item in iter {
            heap.push(item);
        }
        heap
    }
}

impl<T, F, H> Extend<T> for Heap<T, F, H>
where
    T: Ord + fmt::Debug,
    F: Fn(&T, &T) -> Ordering,
    H: HeapType,
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push(item);
        }
    }
}

impl<T, F, H> fmt::Display for Heap<T, F, H>
where
    T: Ord + fmt::Debug,
    F: Fn(&T, &T) -> Ordering,
    H: HeapType,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}(d={}) [", H::type_name(), self.d)?;
        for (i, item) in self.heap.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:?}", item)?;
        }
        write!(f, "]")
    }
}

pub trait HeapCreate<T: Ord + fmt::Debug>: HeapType {
    fn create_heap() -> Heap<T, fn(&T, &T) -> Ordering, Self>;
    fn comparison_fn() -> fn(&T, &T) -> Ordering;
}