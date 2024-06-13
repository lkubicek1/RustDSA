#[cfg(test)]
mod binary_search_tests {

    use crate::search::binary_search::binary_search;

    #[test]
    fn test_binary_search_with_integers() {
        let arr = [1, 2, 3, 4, 5, 6];
        assert_eq!(binary_search(&arr, &4), Some(3));
        assert_eq!(binary_search(&arr, &1), Some(0));
        assert_eq!(binary_search(&arr, &6), Some(5));
        assert_eq!(binary_search(&arr, &7), None);
    }

    #[test]
    fn test_binary_search_with_strings() {
        let arr = ["apple", "banana", "cherry", "date"];
        assert_eq!(binary_search(&arr, &"banana"), Some(1));
        assert_eq!(binary_search(&arr, &"apple"), Some(0));
        assert_eq!(binary_search(&arr, &"date"), Some(3));
        assert_eq!(binary_search(&arr, &"fig"), None);
    }
}
