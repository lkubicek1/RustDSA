/*#[cfg(test)]
mod priority_queue_tests {
    use crate::trees::heaps::min_heap::MinHeap;
    use crate::trees::heaps::priority_queue::PriorityQueue;

    type PQ<T> = PriorityQueue<T, i32, MinHeap<(i32, T)>>;

    #[test]
    fn test_new_queue_is_empty() {
        let queue: PQ<String> = PQ::new(MinHeap::new());
        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn test_push_and_peek() {
        let mut queue = PQ::new(MinHeap::new());
        queue.push("task1".to_string(), 5);
        assert_eq!(queue.peek(), Some(&"task1".to_string()));
        queue.push("task2".to_string(), 3);
        assert_eq!(queue.peek(), Some(&"task2".to_string()));
        queue.push("task3".to_string(), 7);
        assert_eq!(queue.peek(), Some(&"task2".to_string()));
    }

    #[test]
    fn test_pop() {
        let mut queue = PQ::new(MinHeap::new());
        queue.push("task1".to_string(), 5);
        queue.push("task2".to_string(), 3);
        queue.push("task3".to_string(), 7);
        assert_eq!(queue.pop(), Some("task2".to_string()));
        assert_eq!(queue.pop(), Some("task1".to_string()));
        assert_eq!(queue.pop(), Some("task3".to_string()));
        assert_eq!(queue.pop(), None);
    }

    #[test]
    fn test_peek_empty_queue() {
        let queue: PQ<String> = PQ::new(MinHeap::new());
        assert_eq!(queue.peek(), None);
    }

    #[test]
    fn test_pop_empty_queue() {
        let mut queue: PQ<String> = PQ::new(MinHeap::new());
        assert_eq!(queue.pop(), None);
    }

    #[test]
    fn test_queue_ordering() {
        let mut queue = PQ::new(MinHeap::new());
        let tasks = vec![
            ("task1".to_string(), 3),
            ("task2".to_string(), 1),
            ("task3".to_string(), 4),
            ("task4".to_string(), 1),
            ("task5".to_string(), 5),
        ];
        for (task, priority) in tasks.clone() {
            queue.push(task, priority);
        }
        let mut sorted_tasks = tasks;
        sorted_tasks.sort_by_key(|(_, priority)| *priority);
        for (expected_task, _) in sorted_tasks {
            assert_eq!(queue.pop(), Some(expected_task));
        }
        assert_eq!(queue.pop(), None);
    }

    #[test]
    fn test_queue_with_duplicates() {
        let mut queue = PQ::new(MinHeap::new());
        queue.push("task1".to_string(), 5);
        queue.push("task2".to_string(), 5);
        queue.push("task3".to_string(), 3);
        queue.push("task4".to_string(), 5);
        assert_eq!(queue.pop(), Some("task3".to_string()));
        assert_eq!(queue.pop(), Some("task1".to_string())); // or "task2" or "task4"
        assert_eq!(queue.pop(), Some("task2".to_string())); // or "task4"
        assert_eq!(queue.pop(), Some("task4".to_string())); // or "task2"
        assert_eq!(queue.pop(), None);
    }

    #[test]
    fn test_queue_with_negative_priorities() {
        let mut queue = PQ::new(MinHeap::new());
        queue.push("task1".to_string(), -1);
        queue.push("task2".to_string(), -5);
        queue.push("task3".to_string(), -2);
        queue.push("task4".to_string(), -4);
        assert_eq!(queue.pop(), Some("task2".to_string()));
        assert_eq!(queue.pop(), Some("task4".to_string()));
        assert_eq!(queue.pop(), Some("task3".to_string()));
        assert_eq!(queue.pop(), Some("task1".to_string()));
        assert_eq!(queue.pop(), None);
    }

    #[test]
    fn test_queue_peek_with_priority() {
        let mut queue = PQ::new(MinHeap::new());
        queue.push("task1".to_string(), 3);
        queue.push("task2".to_string(), 1);
        queue.push("task3".to_string(), 4);
        assert_eq!(queue.peek_with_priority(), Some((&1, &"task2".to_string())));
        queue.pop();
        assert_eq!(queue.peek_with_priority(), Some((&3, &"task1".to_string())));
    }

    #[test]
    fn test_queue_extend() {
        let mut queue = PQ::new(MinHeap::new());
        queue.push("task1".to_string(), 3);
        queue.extend(vec![
            (1, "task2".to_string()),
            (4, "task3".to_string()),
            (2, "task4".to_string()),
        ]);
        assert_eq!(queue.len(), 4);
        assert_eq!(queue.pop(), Some("task2".to_string()));
        assert_eq!(queue.pop(), Some("task4".to_string()));
        assert_eq!(queue.pop(), Some("task1".to_string()));
        assert_eq!(queue.pop(), Some("task3".to_string()));
    }

    #[test]
    fn test_queue_drain() {
        let mut queue = PQ::new(MinHeap::new());
        queue.extend(vec![
            (3, "task1".to_string()),
            (1, "task2".to_string()),
            (4, "task3".to_string()),
            (2, "task4".to_string()),
        ]);
        let drained: Vec<_> = queue.drain().collect();
        assert!(queue.is_empty());
        assert_eq!(drained, vec!["task2", "task4", "task1", "task3"]);
    }

    #[test]
    fn test_queue_retain() {
        let mut queue = PQ::new(MinHeap::new());
        queue.extend(vec![
            (3, "task1".to_string()),
            (1, "task2".to_string()),
            (4, "task3".to_string()),
            (2, "task4".to_string()),
        ]);
        queue.retain(|&priority, _| priority % 2 == 0);
        assert_eq!(queue.len(), 2);
        assert_eq!(queue.pop(), Some("task4".to_string()));
        assert_eq!(queue.pop(), Some("task3".to_string()));
        assert_eq!(queue.pop(), None);
    }

    #[test]
    fn test_queue_from_iterator() {
        let tasks = vec![
            (3, "task1".to_string()),
            (1, "task2".to_string()),
            (4, "task3".to_string()),
            (2, "task4".to_string()),
        ];
        let mut queue = PQ::new(MinHeap::from_iter(tasks));
        assert_eq!(queue.len(), 4);
        assert_eq!(queue.pop(), Some("task2".to_string()));
        assert_eq!(queue.pop(), Some("task4".to_string()));
        assert_eq!(queue.pop(), Some("task1".to_string()));
        assert_eq!(queue.pop(), Some("task3".to_string()));
    }

    #[test]
    fn test_queue_into_iterator() {
        let mut queue = PQ::new(MinHeap::new());
        queue.extend(vec![
            (3, "task1".to_string()),
            (1, "task2".to_string()),
            (4, "task3".to_string()),
            (2, "task4".to_string()),
        ]);
        let items: Vec<_> = queue.into_iter().collect();
        assert_eq!(items, vec!["task2", "task4", "task1", "task3"]);
    }

    #[test]
    fn test_display_trait() {
        let mut queue = PQ::new(MinHeap::new());
        queue.push("task1".to_string(), 3);
        queue.push("task2".to_string(), 1);
        queue.push("task3".to_string(), 4);
        let display_string = format!("{}", queue);
        assert!(display_string.starts_with("PriorityQueue MinHeap ["));
        assert!(display_string.ends_with("]"));
        assert!(display_string.contains("task1"));
        assert!(display_string.contains("task2"));
        assert!(display_string.contains("task3"));
    }

    #[test]
    fn test_queue_clear() {
        let mut queue = PQ::new(MinHeap::new());
        queue.extend(vec![
            (3, "task1".to_string()),
            (1, "task2".to_string()),
            (4, "task3".to_string()),
            (2, "task4".to_string()),
        ]);
        assert!(!queue.is_empty());
        while queue.pop().is_some() {}
        assert!(queue.is_empty());
    }

    #[test]
    fn test_queue_large_number_of_elements() {
        let mut queue = PQ::new(MinHeap::new());
        let n = 10000;
        for i in (0..n).rev() {
            queue.push(i.to_string(), i);
        }
        assert_eq!(queue.len(), n as usize);
        for i in 0..n {
            assert_eq!(queue.pop(), Some(i.to_string()));
        }
        assert!(queue.is_empty());
    }

    #[test]
    fn test_queue_with_custom_type() {
        #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
        struct Task {
            id: i32,
            description: String,
        }

        let mut queue = PriorityQueue::new(MinHeap::new());
        queue.push(Task { id: 1, description: "Low priority task".to_string() }, 3);
        queue.push(Task { id: 2, description: "High priority task".to_string() }, 1);
        queue.push(Task { id: 3, description: "Medium priority task".to_string() }, 2);

        assert_eq!(queue.pop().map(|t| t.id), Some(2));
        assert_eq!(queue.pop().map(|t| t.id), Some(3));
        assert_eq!(queue.pop().map(|t| t.id), Some(1));
        assert_eq!(queue.pop(), None);
    }
}*/