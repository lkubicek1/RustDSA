#[cfg(test)]
mod indexed_priority_queue_tests {
    use crate::trees::heaps::indexed_priority_queue::IndexedPQ;

    #[test]
    fn test_insert_and_pop() {
        let mut pq = IndexedPQ::new();
        assert!(pq.insert("a", 3));
        assert!(pq.insert("b", 1));
        assert!(pq.insert("c", 2));

        assert_eq!(pq.pop(), Some(("b", 1)));
        assert_eq!(pq.pop(), Some(("c", 2)));
        assert_eq!(pq.pop(), Some(("a", 3)));
        assert_eq!(pq.pop(), None);
    }

    #[test]
    fn test_decrease_key() {
        let mut pq = IndexedPQ::new();
        pq.insert("a", 3);
        pq.insert("b", 2);
        pq.insert("c", 1);

        assert!(pq.decrease_key(&"a", 0));
        assert_eq!(pq.pop(), Some(("a", 0)));
    }

    #[test]
    fn test_peek() {
        let mut pq = IndexedPQ::new();
        pq.insert("a", 2);
        pq.insert("b", 1);

        assert_eq!(pq.peek(), Some((&"b", &1)));
        assert_eq!(pq.len(), 2);
    }

    #[test]
    fn test_contains() {
        let mut pq = IndexedPQ::new();
        pq.insert("a", 1);

        assert!(pq.contains(&"a"));
        assert!(!pq.contains(&"b"));
    }

    #[test]
    fn test_is_empty_and_len() {
        let mut pq = IndexedPQ::new();
        assert!(pq.is_empty());
        assert_eq!(pq.len(), 0);

        pq.insert("a", 1);
        assert!(!pq.is_empty());
        assert_eq!(pq.len(), 1);
    }

    #[test]
    fn test_duplicate_insert() {
        let mut pq = IndexedPQ::new();
        assert!(pq.insert("a", 1));
        assert!(!pq.insert("a", 2));
        assert_eq!(pq.len(), 1);
    }

    #[test]
    fn test_decrease_key_nonexistent() {
        let mut pq = IndexedPQ::new();
        pq.insert("a", 2);
        assert!(!pq.decrease_key(&"b", 1));
    }

    #[test]
    fn test_large_dataset() {
        let mut pq = IndexedPQ::new();
        for i in 0..1000 {
            pq.insert(i, 1000 - i);
        }

        for i in 0..1000 {
            assert_eq!(pq.pop(), Some(((999 - i), i + 1)));
        }
    }
}