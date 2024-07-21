#[cfg(test)]
mod min_heap_tests {
    use crate::trees::heaps::heap::Heap;
    use crate::trees::heaps::min_heap::MinHeap;

    #[test]
    fn test_new_heap_is_empty() {
        let heap: MinHeap<i32> = MinHeap::new();
        assert!(heap.is_empty());
        assert_eq!(heap.len(), 0);
    }

    #[test]
    fn test_push_and_peek() {
        let mut heap = MinHeap::new();
        heap.push(5);
        assert_eq!(heap.peek(), Some(&5));
        heap.push(3);
        assert_eq!(heap.peek(), Some(&3));
        heap.push(7);
        assert_eq!(heap.peek(), Some(&3));
    }

    #[test]
    fn test_pop() {
        let mut heap = MinHeap::new();
        heap.push(5);
        heap.push(3);
        heap.push(7);
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(7));
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_peek_empty_heap() {
        let heap: MinHeap<i32> = MinHeap::new();
        assert_eq!(heap.peek(), None);
    }

    #[test]
    fn test_pop_empty_heap() {
        let mut heap: MinHeap<i32> = MinHeap::new();
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_heap_ordering() {
        let mut heap = MinHeap::new();
        let values = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        for &value in &values {
            heap.push(value);
        }
        let mut sorted = values;
        sorted.sort();
        for expected in sorted {
            assert_eq!(heap.pop(), Some(expected));
        }
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_heap_with_duplicates() {
        let mut heap = MinHeap::new();
        heap.push(5);
        heap.push(5);
        heap.push(3);
        heap.push(5);
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_heap_with_negative_numbers() {
        let mut heap = MinHeap::new();
        heap.push(-1);
        heap.push(-5);
        heap.push(-2);
        heap.push(-4);
        assert_eq!(heap.pop(), Some(-5));
        assert_eq!(heap.pop(), Some(-4));
        assert_eq!(heap.pop(), Some(-2));
        assert_eq!(heap.pop(), Some(-1));
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_heap_clear() {
        let mut heap = MinHeap::new();
        heap.push(1);
        heap.push(2);
        heap.push(3);
        assert!(!heap.is_empty());
        while heap.pop().is_some() {}
        assert!(heap.is_empty());
    }

    #[test]
    fn test_heap_peek_does_not_remove() {
        let mut heap = MinHeap::new();
        heap.push(1);
        heap.push(2);
        assert_eq!(heap.peek(), Some(&1));
        assert_eq!(heap.peek(), Some(&1));
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.peek(), Some(&2));
    }

    #[test]
    fn test_heap_large_number_of_elements() {
        let mut heap = MinHeap::new();
        let n = 10000;
        for i in (0..n).rev() {
            heap.push(i);
        }
        assert_eq!(heap.len(), n);
        for i in 0..n {
            assert_eq!(heap.pop(), Some(i));
        }
        assert!(heap.is_empty());
    }

    #[test]
    fn test_heap_from_iterator() {
        let values = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        let mut heap: MinHeap<i32> = values.iter().cloned().collect();
        let mut sorted = values.clone();
        sorted.sort(); // sort in ascending order
        for expected in sorted {
            assert_eq!(heap.pop(), Some(expected));
        }
    }

    #[test]
    fn test_heap_into_vec() {
        let mut heap = MinHeap::new();
        heap.push(3);
        heap.push(1);
        heap.push(4);
        let vec: Vec<i32> = heap.into();
        assert_eq!(vec, vec![1, 3, 4]);
    }

    #[test]
    fn test_heap_extend() {
        let mut heap = MinHeap::new();
        heap.push(4);
        heap.extend(vec![3, 2, 1]);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), Some(4));
    }

    #[test]
    fn test_heap_drain() {
        let mut heap = MinHeap::new();
        heap.extend(vec![3, 1, 4, 1, 5, 9, 2, 6]);
        let drained: Vec<i32> = heap.drain().collect();
        assert!(heap.is_empty());
        assert_eq!(drained, vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn test_heap_retain() {
        let mut heap = MinHeap::new();
        heap.extend(1..=10);
        heap.retain(|&x| x % 2 == 0);
        assert_eq!(heap.len(), 5);
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.pop(), Some(6));
        assert_eq!(heap.pop(), Some(8));
        assert_eq!(heap.pop(), Some(10));
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_heap_with_custom_type() {
        #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
        struct CustomType(i32);

        let mut heap = MinHeap::new();
        heap.push(CustomType(5));
        heap.push(CustomType(3));
        heap.push(CustomType(7));
        assert_eq!(heap.pop(), Some(CustomType(3)));
        assert_eq!(heap.pop(), Some(CustomType(5)));
        assert_eq!(heap.pop(), Some(CustomType(7)));
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_display_trait() {
        let mut heap = MinHeap::new();
        heap.push(3);
        heap.push(1);
        heap.push(4);
        let display_string = format!("{}", heap);
        assert!(display_string.contains("MinHeap"));
        assert!(display_string.contains("1"));
        assert!(display_string.contains("3"));
        assert!(display_string.contains("4"));
    }
}