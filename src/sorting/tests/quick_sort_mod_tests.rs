#[cfg(test)]
mod quick_sort_tests {
    use crate::sorting::quick_sort_mod::quick_sort;

    #[test]
    fn test_sort_unsorted_array() {
        let mut arr = vec![33, 18, 78, 64, 45, 32, 70, 11, 27];
        quick_sort(&mut arr, 10);
        assert_eq!(arr, vec![11, 18, 27, 32, 33, 45, 64, 70, 78]);
    }

    #[test]
    fn test_sort_ascending_array() {
        let mut arr = vec![10, 20, 30, 40, 50, 60, 70, 80, 90];
        quick_sort(&mut arr, 10);
        assert_eq!(arr, vec![10, 20, 30, 40, 50, 60, 70, 80, 90]);
    }

    #[test]
    fn test_sort_descending_array() {
        let mut arr = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        quick_sort(&mut arr, 10);
        assert_eq!(arr, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }

    #[test]
    fn test_sort_single_element() {
        let mut arr = [1];
        quick_sort(&mut arr, 10);
        assert_eq!(arr, [1]);
    }

    #[test]
    fn test_sort_empty_array() {
        let mut arr: [i32; 0] = [];
        quick_sort(&mut arr, 10);
        assert_eq!(arr, []);
    }

    #[test]
    fn test_sort_unsorted_strings() {
        let mut arr = vec!["banana", "apple", "cherry", "date"];
        quick_sort(&mut arr, 10);
        assert_eq!(arr, vec!["apple", "banana", "cherry", "date"]);
    }

    #[test]
    fn test_sort_ascending_strings() {
        let mut arr = vec!["apple", "banana", "cherry", "date"];
        quick_sort(&mut arr, 10);
        assert_eq!(arr, vec!["apple", "banana", "cherry", "date"]);
    }

    #[test]
    fn test_sort_descending_strings() {
        let mut arr = vec!["date", "cherry", "banana", "apple"];
        quick_sort(&mut arr, 10);
        assert_eq!(arr, vec!["apple", "banana", "cherry", "date"]);
    }

    #[test]
    fn test_sort_unsorted_chars() {
        let mut arr = vec!['d', 'b', 'a', 'c'];
        quick_sort(&mut arr, 10);
        assert_eq!(arr, vec!['a', 'b', 'c', 'd']);
    }

    #[test]
    fn test_sort_ascending_chars() {
        let mut arr = vec!['a', 'b', 'c', 'd'];
        quick_sort(&mut arr, 10);
        assert_eq!(arr, vec!['a', 'b', 'c', 'd']);
    }

    #[test]
    fn test_sort_descending_chars() {
        let mut arr = vec!['d', 'c', 'b', 'a'];
        quick_sort(&mut arr, 10);
        assert_eq!(arr, vec!['a', 'b', 'c', 'd']);
    }
}
