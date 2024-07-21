#[cfg(test)]
mod priority_queue_tests {
    use crate::trees::heaps::priority_queue::{MaxPriorityQueue, MinPriorityQueue};

    #[test]
    fn test_max_priority_queue() {
        let mut pq = MaxPriorityQueue::new();
        pq.push("task1", 3);
        pq.push("task2", 1);
        pq.push("task3", 4);
        pq.push("task4", 1);
        pq.push("task5", 5);

        assert_eq!(pq.pop(), Some("task5"));
        assert_eq!(pq.pop(), Some("task3"));
        assert_eq!(pq.pop(), Some("task1"));
        assert_eq!(pq.pop_with_priority(), Some((1, "task4")));
        assert_eq!(pq.pop_with_priority(), Some((1, "task2")));
        assert_eq!(pq.pop(), None);
    }

    #[test]
    fn test_min_priority_queue() {
        let mut pq: MinPriorityQueue<&str, i32> = MinPriorityQueue::new();
        pq.push("task1", 3);
        pq.push("task2", 1);
        pq.push("task3", 4);
        pq.push("task4", 1);
        pq.push("task5", 5);

        assert_eq!(pq.pop(), Some("task2")); // Lowest priority, inserted first
        assert_eq!(pq.pop(), Some("task4")); // Same lowest priority, inserted second
        assert_eq!(pq.pop(), Some("task1"));
        assert_eq!(pq.pop(), Some("task3"));
        assert_eq!(pq.pop(), Some("task5"));
        assert_eq!(pq.pop(), None);
    }

    #[test]
    fn test_priority_queue_retain() {
        let mut pq = MaxPriorityQueue::new();
        pq.push("task1", 3);
        pq.push("task2", 1);
        pq.push("task3", 4);
        pq.push("task4", 2);

        pq.retain(|&priority, _| priority > 2);

        assert_eq!(pq.len(), 2);
        assert_eq!(pq.pop(), Some("task3"));
        assert_eq!(pq.pop(), Some("task1"));
    }

    #[test]
    fn test_priority_queue_into_sorted_vec() {
        let pq: MinPriorityQueue<&str, i32> = vec![(3, "task1"), (1, "task2"), (4, "task3"), (2, "task4")]
            .into_iter()
            .collect();
        let sorted = pq.into_sorted_vec();
        assert_eq!(sorted, vec![(1, "task2"), (2, "task4"), (3, "task1"), (4, "task3")]);

        let pq: MaxPriorityQueue<&str, i32> = vec![(3, "task1"), (1, "task2"), (4, "task3"), (2, "task4")]
            .into_iter()
            .collect();
        let sorted = pq.into_sorted_vec();
        assert_eq!(sorted, vec![(4, "task3"), (3, "task1"), (2, "task4"), (1, "task2")]);
    }

    #[test]
    fn test_priority_queue_clear() {
        let mut pq = MaxPriorityQueue::new();
        pq.push("task1", 3);
        pq.push("task2", 1);
        assert_eq!(pq.len(), 2);

        pq.clear();
        assert_eq!(pq.len(), 0);
        assert!(pq.is_empty());

        pq.push("task3", 2);
        assert_eq!(pq.pop(), Some("task3"));
    }

    #[test]
    fn test_priority_queue_drain() {
        let mut pq = MinPriorityQueue::new();
        pq.push("task1", 3);
        pq.push("task2", 1);
        pq.push("task3", 2);

        let drained: Vec<_> = pq.drain().collect();
        assert_eq!(drained, vec!["task2", "task3", "task1"]);
        assert!(pq.is_empty());
    }
}
