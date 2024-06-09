#[cfg(test)]
mod tests {
    use crate::arrays::reverse_array::reverse_array;

    #[test]
    fn test_reverse_array_with_integers_even_length() {
        let mut arr = [1, 2, 3, 4, 5, 6];
        reverse_array(&mut arr);
        assert_eq!(arr, [6, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_reverse_array_with_integers_odd_length() {
        let mut arr = [1, 2, 3, 4, 5];
        reverse_array(&mut arr);
        assert_eq!(arr, [5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_reverse_array_with_strings_even_length() {
        let mut arr = ["apple", "banana", "cherry", "date"];
        reverse_array(&mut arr);
        assert_eq!(arr, ["date", "cherry", "banana", "apple"]);
    }

    #[test]
    fn test_reverse_array_with_strings_odd_length() {
        let mut arr = ["apple", "banana", "cherry"];
        reverse_array(&mut arr);
        assert_eq!(arr, ["cherry", "banana", "apple"]);
    }

    #[test]
    fn test_reverse_array_with_empty_array() {
        let mut arr: [i32; 0] = [];
        reverse_array(&mut arr);
        assert_eq!(arr, []);
    }

    #[test]
    fn test_reverse_array_with_single_element() {
        let mut arr = [1];
        reverse_array(&mut arr);
        assert_eq!(arr, [1]);
    }

    #[test]
    fn test_reverse_array_with_multiple_types() {
        let mut int_arr = [1, 2, 3];
        reverse_array(&mut int_arr);
        assert_eq!(int_arr, [3, 2, 1]);

        let mut str_arr = ["a", "b", "c"];
        reverse_array(&mut str_arr);
        assert_eq!(str_arr, ["c", "b", "a"]);

        let mut float_arr = [1.1, 2.2, 3.3];
        reverse_array(&mut float_arr);
        assert_eq!(float_arr, [3.3, 2.2, 1.1]);
    }
}