use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use crate::trees::heaps::heap::{Heap, HeapType, GenericHeap};

pub trait IndexedHeapTrait<T>: Heap<T>
where
    T: Ord + fmt::Debug + Clone + Hash,
{
    fn get_index(&self, item: &T) -> Option<usize>;
    fn remove(&mut self, item: &T) -> Option<T>;
    fn update(&mut self, old_item: T, new_item: T);
    fn decrease_key(&mut self, item: &T, new_value: T);
    fn increase_key(&mut self, item: &T, new_value: T);
}

pub struct IndexedHeap<T, H>
where
    T: Ord + fmt::Debug + Clone + Hash,
    H: HeapType,
{
    heap: GenericHeap<T, H>,
    index_map: HashMap<T, usize>,
    position_map: Vec<usize>,
}

impl<T, H> IndexedHeap<T, H>
where
    T: Ord + fmt::Debug + Clone + Hash,
    H: HeapType,
{
    pub fn new() -> Self {
        Self {
            heap: GenericHeap::new(),
            index_map: HashMap::new(),
            position_map: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            heap: GenericHeap::with_capacity(capacity),
            index_map: HashMap::with_capacity(capacity),
            position_map: Vec::with_capacity(capacity),
        }
    }

    fn update_indices(&mut self) {
        for (index, item) in self.heap.heap.iter().enumerate() {
            self.index_map.insert(item.clone(), index);
        }
    }
}

impl<T, H> Heap<T> for IndexedHeap<T, H>
where
    T: Ord + fmt::Debug + Clone + Hash,
    H: HeapType,
{
    fn new() -> Self {
        Self::new()
    }

    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }

    fn push(&mut self, item: T) {
        let index = self.heap.len();
        self.heap.push(item.clone());
        self.index_map.insert(item.clone(), index);
        if index < self.position_map.len() {
            self.position_map[index] = index;
        } else {
            self.position_map.push(index);
        }
        self.update_indices();
    }

    fn pop(&mut self) -> Option<T> {
        if let Some(item) = self.heap.pop() {
            self.index_map.remove(&item);
            self.update_indices();
            Some(item)
        } else {
            None
        }
    }

    fn peek(&self) -> Option<&T> {
        self.heap.peek()
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

    fn retain<P>(&mut self, predicate: P)
    where
        P: FnMut(&T) -> bool,
    {
        self.heap.retain(predicate);
        self.update_indices();
    }

    fn clear(&mut self) {
        self.heap.clear();
        self.index_map.clear();
        self.position_map.clear();
    }
}

impl<T, H> IndexedHeapTrait<T> for IndexedHeap<T, H>
where
    T: Ord + fmt::Debug + Clone + Hash,
    H: HeapType,
{
    fn get_index(&self, item: &T) -> Option<usize> {
        self.index_map.get(item).copied()
    }

    fn remove(&mut self, item: &T) -> Option<T> {
        if let Some(&index) = self.index_map.get(item) {
            let last = self.heap.heap.pop().unwrap();
            if index < self.heap.len() {
                let removed = std::mem::replace(&mut self.heap.heap[index], last.clone());
                self.index_map.remove(&removed);
                self.index_map.insert(last, index);
                self.heap.heapify_down(index);
                self.heap.heapify_up(index);
                Some(removed)
            } else {
                self.index_map.remove(item);
                Some(last)
            }
        } else {
            None
        }
    }

    fn update(&mut self, old_item: T, new_item: T) {
        if let Some(index) = self.index_map.remove(&old_item) {
            self.heap.heap[index] = new_item.clone();
            self.index_map.insert(new_item, index);
            self.heap.heapify_down(index);
            self.heap.heapify_up(index);
        }
    }

    fn decrease_key(&mut self, item: &T, new_value: T) {
        if let Some(&index) = self.index_map.get(item) {
            if new_value < self.heap.heap[index] {
                self.heap.heap[index] = new_value.clone();
                self.index_map.insert(new_value.clone(), index);
                self.index_map.remove(item);
                self.heap.heapify_up(index);
            }
        }
    }

    fn increase_key(&mut self, item: &T, new_value: T) {
        if let Some(&index) = self.index_map.get(item) {
            if new_value > self.heap.heap[index] {
                self.heap.heap[index] = new_value.clone();
                self.index_map.insert(new_value.clone(), index);
                self.index_map.remove(item);
                self.heap.heapify_down(index);
            }
        }
    }
}

impl<T, H> fmt::Display for IndexedHeap<T, H>
where
    T: Ord + fmt::Debug + Clone + Hash,
    H: HeapType,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}[", H::type_name())?;
        for (i, item) in self.heap.heap.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:?}", item)?;
        }
        write!(f, "]")
    }
}
