#[cfg(test)]
mod merge_sort_tests {
    use crate::sorting::merge_sort::MergeSort;

    #[test]
    fn test_sort_unsorted_array() {
        let mut arr = vec![33, 18, 78, 64, 45, 32, 70, 11, 27];
        let merge_sort = MergeSort::new();
        merge_sort.sort(&mut arr);
        assert_eq!(arr, vec![11, 18, 27, 32, 33, 45, 64, 70, 78]);
    }

    #[test]
    fn test_sort_ascending_array() {
        let mut arr = vec![10, 20, 30, 40, 50, 60, 70, 80, 90];
        let merge_sort = MergeSort::new();
        merge_sort.sort(&mut arr);
        assert_eq!(arr, vec![10, 20, 30, 40, 50, 60, 70, 80, 90]);
    }

    #[test]
    fn test_sort_descending_array() {
        let mut arr = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        let merge_sort = MergeSort::new();
        merge_sort.sort(&mut arr);
        assert_eq!(arr, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }

    #[test]
    fn test_sort_single_element() {
        let mut arr = [1];
        let merge_sort = MergeSort::new();
        merge_sort.sort(&mut arr);
        assert_eq!(arr, [1]);
    }

    #[test]
    fn test_sort_empty_array() {
        let mut arr: [i32; 0] = [];
        let merge_sort = MergeSort::new();
        merge_sort.sort(&mut arr);
        assert_eq!(arr, []);
    }

    #[test]
    fn test_sort_unsorted_strings() {
        let mut arr = vec!["banana", "apple", "cherry", "date"];
        let merge_sort = MergeSort::new();
        merge_sort.sort(&mut arr);
        assert_eq!(arr, vec!["apple", "banana", "cherry", "date"]);
    }

    #[test]
    fn test_sort_ascending_strings() {
        let mut arr = vec!["apple", "banana", "cherry", "date"];
        let merge_sort = MergeSort::new();
        merge_sort.sort(&mut arr);
        assert_eq!(arr, vec!["apple", "banana", "cherry", "date"]);
    }

    #[test]
    fn test_sort_descending_strings() {
        let mut arr = vec!["date", "cherry", "banana", "apple"];
        let merge_sort = MergeSort::new();
        merge_sort.sort(&mut arr);
        assert_eq!(arr, vec!["apple", "banana", "cherry", "date"]);
    }

    #[test]
    fn test_sort_unsorted_chars() {
        let mut arr = vec!['d', 'b', 'a', 'c'];
        let merge_sort = MergeSort::new();
        merge_sort.sort(&mut arr);
        assert_eq!(arr, vec!['a', 'b', 'c', 'd']);
    }

    #[test]
    fn test_sort_ascending_chars() {
        let mut arr = vec!['a', 'b', 'c', 'd'];
        let merge_sort = MergeSort::new();
        merge_sort.sort(&mut arr);
        assert_eq!(arr, vec!['a', 'b', 'c', 'd']);
    }

    #[test]
    fn test_sort_descending_chars() {
        let mut arr = vec!['d', 'c', 'b', 'a'];
        let merge_sort = MergeSort::new();
        merge_sort.sort(&mut arr);
        assert_eq!(arr, vec!['a', 'b', 'c', 'd']);
    }
}
