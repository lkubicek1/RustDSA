use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;

#[derive(Debug)]
pub struct IndexedPQ<K, P>
where
    K: Eq + Hash + Clone + Ord,
    P: Ord,
{
    heap: Vec<PQItem<K, P>>,
    index_map: HashMap<K, usize>,
}

#[derive(Debug, Eq, PartialEq)]
struct PQItem<K, P>
where
    K: Eq + Hash + Ord,
    P: Ord,
{
    priority: P,
    key: K,
}

impl<K, P> Ord for PQItem<K, P>
where
    K: Eq + Hash + Ord,
    P: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority)
            .then_with(|| self.key.cmp(&other.key))
    }
}

impl<K, P> PartialOrd for PQItem<K, P>
where
    K: Eq + Hash + Ord,
    P: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K, P> IndexedPQ<K, P>
where
    K: Eq + Hash + Clone + Ord,
    P: Ord + Clone,
{
    pub fn new() -> Self {
        IndexedPQ {
            heap: Vec::new(),
            index_map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, priority: P) -> bool {
        if self.index_map.contains_key(&key) {
            return false;
        }
        let index = self.heap.len();
        self.heap.push(PQItem { priority, key: key.clone() });
        self.index_map.insert(key, index);
        self.sift_up(index);
        true
    }

    pub fn decrease_key(&mut self, key: &K, new_priority: P) -> bool {
        if let Some(&index) = self.index_map.get(key) {
            if new_priority < self.heap[index].priority {
                self.heap[index].priority = new_priority;
                self.sift_up(index);
            }
            true
        } else {
            false
        }
    }

    pub fn pop(&mut self) -> Option<(K, P)> {
        if self.heap.is_empty() {
            return None;
        }

        let last_idx = self.heap.len() - 1;
        self.heap.swap(0, last_idx);
        let PQItem { priority, key } = self.heap.pop().unwrap();
        self.index_map.remove(&key);

        if !self.heap.is_empty() {
            self.sift_down(0);
        }

        Some((key, priority))
    }

    pub fn peek(&self) -> Option<(&K, &P)> {
        self.heap.first().map(|item| (&item.key, &item.priority))
    }

    pub fn contains(&self, key: &K) -> bool {
        self.index_map.contains_key(key)
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }

    fn sift_up(&mut self, mut index: usize) {
        while index > 0 {
            let parent = (index - 1) / 2;
            if self.heap[index] <= self.heap[parent] {
                break;
            }
            self.heap.swap(index, parent);
            *self.index_map.get_mut(&self.heap[index].key).unwrap() = index;
            *self.index_map.get_mut(&self.heap[parent].key).unwrap() = parent;
            index = parent;
        }
    }

    fn sift_down(&mut self, mut index: usize) {
        let last = self.heap.len() - 1;
        loop {
            let left = 2 * index + 1;
            let right = left + 1;
            let mut largest = index;

            if left <= last && self.heap[left] > self.heap[largest] {
                largest = left;
            }
            if right <= last && self.heap[right] > self.heap[largest] {
                largest = right;
            }

            if largest == index {
                break;
            }

            self.heap.swap(index, largest);
            *self.index_map.get_mut(&self.heap[index].key).unwrap() = index;
            *self.index_map.get_mut(&self.heap[largest].key).unwrap() = largest;
            index = largest;
        }
    }
}