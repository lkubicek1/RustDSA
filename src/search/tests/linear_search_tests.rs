#[cfg(test)]
mod linear_search_tests {
    use crate::search::linear_search::linear_search;

    #[test]
    fn test_linear_search_with_ints() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(linear_search(&arr, &3), Some(2));
        assert_eq!(linear_search(&arr, &6), None);
    }

    #[test]
    fn test_linear_search_with_strings() {
        let arr = ["apple", "banana", "cherry"];
        assert_eq!(linear_search(&arr, &"banana"), Some(1));
        assert_eq!(linear_search(&arr, &"orange"), None);
    }
}
