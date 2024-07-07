#[cfg(test)]
mod insertion_sort_with_tracking_tests {
    use crate::sorting::sort_tracker::SortTracker;
    use crate::sorting::insertion_sort_with_tracking::insertion_sort;

    #[test]
    fn test_sort_unsorted_array() {
        let mut arr = vec![33, 18, 78, 64, 45, 32, 70, 11, 27];
        let mut tracker = SortTracker::new();
        insertion_sort(&mut arr, &mut tracker);
        assert_eq!(arr, vec![11, 18, 27, 32, 33, 45, 64, 70, 78]);
        assert_eq!(tracker.get_comparison_count(), 28);
        assert_eq!(tracker.get_swap_count(), 22);
    }

    #[test]
    fn test_sort_ascending_array() {
        let mut arr = vec![10, 20, 30, 40, 50, 60, 70, 80, 90];
        let mut tracker = SortTracker::new();
        insertion_sort(&mut arr, &mut tracker);
        assert_eq!(arr, vec![10, 20, 30, 40, 50, 60, 70, 80, 90]);
        assert_eq!(tracker.get_comparison_count(), 8);
        assert_eq!(tracker.get_swap_count(), 0);
    }

    #[test]
    fn test_sort_descending_array() {
        let mut arr = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        let mut tracker = SortTracker::new();
        insertion_sort(&mut arr, &mut tracker);
        assert_eq!(arr, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
        assert_eq!(tracker.get_comparison_count(), 36);
        assert_eq!(tracker.get_swap_count(), 36);
    }

    #[test]
    fn test_sort_single_element() {
        let mut arr = [1];
        let mut tracker = SortTracker::new();
        insertion_sort(&mut arr, &mut tracker);
        assert_eq!(arr, [1]);
        assert_eq!(tracker.get_comparison_count(), 0);
        assert_eq!(tracker.get_swap_count(), 0);
    }

    #[test]
    fn test_sort_empty_array() {
        let mut arr: [i32; 0] = [];
        let mut tracker = SortTracker::new();
        insertion_sort(&mut arr, &mut tracker);
        assert_eq!(arr, []);
        assert_eq!(tracker.get_comparison_count(), 0);
        assert_eq!(tracker.get_swap_count(), 0);
    }

    #[test]
    fn test_sort_unsorted_strings() {
        let mut arr = vec!["banana", "apple", "cherry", "date"];
        let mut tracker = SortTracker::new();
        insertion_sort(&mut arr, &mut tracker);
        assert_eq!(arr, vec!["apple", "banana", "cherry", "date"]);
        assert_eq!(tracker.get_comparison_count(), 3);
        assert_eq!(tracker.get_swap_count(), 1);
    }

    #[test]
    fn test_sort_ascending_strings() {
        let mut arr = vec!["apple", "banana", "cherry", "date"];
        let mut tracker = SortTracker::new();
        insertion_sort(&mut arr, &mut tracker);
        assert_eq!(arr, vec!["apple", "banana", "cherry", "date"]);
        assert_eq!(tracker.get_comparison_count(), 3);
        assert_eq!(tracker.get_swap_count(), 0);
    }

    #[test]
    fn test_sort_descending_strings() {
        let mut arr = vec!["date", "cherry", "banana", "apple"];
        let mut tracker = SortTracker::new();
        insertion_sort(&mut arr, &mut tracker);
        assert_eq!(arr, vec!["apple", "banana", "cherry", "date"]);
        assert_eq!(tracker.get_comparison_count(), 6);
        assert_eq!(tracker.get_swap_count(), 6);
    }

    #[test]
    fn test_sort_unsorted_chars() {
        let mut arr = vec!['d', 'b', 'a', 'c'];
        let mut tracker = SortTracker::new();
        insertion_sort(&mut arr, &mut tracker);
        assert_eq!(arr, vec!['a', 'b', 'c', 'd']);
        assert_eq!(tracker.get_comparison_count(), 5);
        assert_eq!(tracker.get_swap_count(), 4);
    }

    #[test]
    fn test_sort_ascending_chars() {
        let mut arr = vec!['a', 'b', 'c', 'd'];
        let mut tracker = SortTracker::new();
        insertion_sort(&mut arr, &mut tracker);
        assert_eq!(arr, vec!['a', 'b', 'c', 'd']);
        assert_eq!(tracker.get_comparison_count(), 3);
        assert_eq!(tracker.get_swap_count(), 0);
    }

    #[test]
    fn test_sort_descending_chars() {
        let mut arr = vec!['d', 'c', 'b', 'a'];
        let mut tracker = SortTracker::new();
        insertion_sort(&mut arr, &mut tracker);
        assert_eq!(arr, vec!['a', 'b', 'c', 'd']);
        assert_eq!(tracker.get_comparison_count(), 6);
        assert_eq!(tracker.get_swap_count(), 6);
    }
}
