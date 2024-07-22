#[cfg(test)]
mod indexed_heap_tests {
    use crate::trees::heaps::heap::Heap;
    use crate::trees::heaps::indexed_heap::{IndexedHeap, IndexedHeapTrait};
    use crate::trees::heaps::max_heap::MaxHeapType;
    use crate::trees::heaps::min_heap::MinHeapType;

    #[test]
    fn test_push_and_pop() {
        let mut heap = IndexedHeap::<i32, MinHeapType>::new();
        heap.push(10);
        heap.push(20);
        heap.push(5);

        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(10));
        assert_eq!(heap.pop(), Some(20));
        assert!(heap.pop().is_none());
    }

    #[test]
    fn test_peek() {
        let mut heap = IndexedHeap::<i32, MinHeapType>::new();
        heap.push(10);
        heap.push(20);
        heap.push(5);

        assert_eq!(heap.peek(), Some(&5));
        heap.pop();
        assert_eq!(heap.peek(), Some(&10));
        heap.pop();
        assert_eq!(heap.peek(), Some(&20));
        heap.pop();
        assert!(heap.peek().is_none());
    }

    #[test]
    fn test_len_and_is_empty() {
        let mut heap = IndexedHeap::<i32, MinHeapType>::new();
        assert_eq!(heap.len(), 0);
        assert!(heap.is_empty());

        heap.push(10);
        assert_eq!(heap.len(), 1);
        assert!(!heap.is_empty());

        heap.push(20);
        assert_eq!(heap.len(), 2);

        heap.pop();
        assert_eq!(heap.len(), 1);

        heap.pop();
        assert_eq!(heap.len(), 0);
        assert!(heap.is_empty());
    }

    #[test]
    fn test_remove() {
        let mut heap = IndexedHeap::<i32, MinHeapType>::new();
        heap.push(10);
        heap.push(20);
        heap.push(5);

        assert_eq!(heap.remove(&10), Some(10));
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(20));
        assert!(heap.pop().is_none());
    }

    #[test]
    fn test_update() {
        let mut heap = IndexedHeap::<i32, MinHeapType>::new();
        heap.push(10);
        heap.push(20);
        heap.push(5);

        heap.update(10, 15);
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(15));
        assert_eq!(heap.pop(), Some(20));
        assert!(heap.pop().is_none());
    }

    #[test]
    fn test_decrease_key() {
        let mut heap = IndexedHeap::<i32, MinHeapType>::new();
        heap.push(10);
        heap.push(20);
        heap.push(15);

        heap.decrease_key(&15, 5);
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(10));
        assert_eq!(heap.pop(), Some(20));
        assert!(heap.pop().is_none());
    }

    #[test]
    fn test_increase_key() {
        let mut heap = IndexedHeap::<i32, MinHeapType>::new();
        heap.push(10);
        heap.push(20);
        heap.push(5);

        heap.increase_key(&5, 25);
        assert_eq!(heap.pop(), Some(10));
        assert_eq!(heap.pop(), Some(20));
        assert_eq!(heap.pop(), Some(25));
        assert!(heap.pop().is_none());
    }

    #[test]
    fn test_clear() {
        let mut heap = IndexedHeap::<i32, MinHeapType>::new();
        heap.push(10);
        heap.push(20);
        heap.push(5);

        heap.clear();
        assert_eq!(heap.len(), 0);
        assert!(heap.is_empty());
        assert!(heap.pop().is_none());
    }

    #[test]
    fn test_max_heap() {
        let mut heap = IndexedHeap::<i32, MaxHeapType>::new();
        heap.push(10);
        heap.push(20);
        heap.push(5);

        assert_eq!(heap.pop(), Some(20));
        assert_eq!(heap.pop(), Some(10));
        assert_eq!(heap.pop(), Some(5));
        assert!(heap.pop().is_none());
    }

    #[test]
    fn test_drain() {
        let mut heap = IndexedHeap::<i32, MinHeapType>::new();
        heap.push(10);
        heap.push(20);
        heap.push(5);

        let mut drained: Vec<i32> = heap.drain().collect();
        drained.sort();
        assert_eq!(drained, vec![5, 10, 20]);
        assert!(heap.is_empty());
    }

    #[test]
    fn test_retain() {
        let mut heap = IndexedHeap::<i32, MinHeapType>::new();
        heap.push(10);
        heap.push(20);
        heap.push(5);

        heap.retain(|&x| x != 10);
        assert_eq!(heap.len(), 2);
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(20));
        assert!(heap.pop().is_none());
    }
}
