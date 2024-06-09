# RustDSA

RustDSA is a collection of data structures and algorithms implemented in Rust. The goal of this project is to provide a comprehensive learning resource for Rust programming while exploring common data structures and algorithms.

## Rust Documentation

For more information about Rust and its features, refer to the [Rust documentation](https://doc.rust-lang.org/).

## Table of Contents

1. [Project Structure](#project-structure)
2. [Algorithms and Data Structures](#algorithms-and-data-structures)
   - [Linear Search](#linear-search)
3. [Rust Concepts](#rust-concepts)
4. [Getting Started](#getting-started)
5. [License](#license)

## Project Structure

The project is organized into modules, each focusing on a specific data structure or algorithm:

```
rust_dsa
├── Cargo.toml
├── src
│   ├── lib.rs
│   ├── main.rs
│   ├── search
│   │   ├── mod.rs
│   │   ├── linear_search.rs
│   │   ├── tests
│   │   │   ├── mod.rs
│   │   │   ├── linear_search.rs
│   ├── sorting
│   │   ├── mod.rs
│   │   ├── selection_sort.rs
│   │   ├── insertion_sort.rs
│   │   ├── tests.rs
│   ├── data_structures
│   │   ├── mod.rs
│   │   ├── linked_list.rs
│   │   ├── hashmap.rs
│   │   ├── tests.rs
    ...
```

[Back to Table of Contents](#table-of-contents)

## Algorithms and Data Structures

### Linear Search

Linear search is a simple search algorithm that checks each element in a list until the target element is found or the list ends. The implementation leverages Rust's safety and performance features.

```rust
pub fn linear_search<T: PartialEq>(arr: &[T], target: &T) -> Option<usize> {
    for i in 0..arr.len() {
        if &arr[i] == target {
            return Some(i);
        }
    }
    None
}
```

#### Complexity Analysis

- **Best Case**: O(1) - The target element is found at the first position.
- **Worst Case**: O(n) - The target element is not in the list or is at the last position.
- **Average Case**: O(n) - On average, the algorithm will need to check half of the elements.

#### Test Cases

Tests are provided to ensure the correctness of the linear search implementation.

```rust
#[cfg(test)]
mod tests {
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
```

[Back to Table of Contents](#table-of-contents)

## Rust Concepts

### Ownership and Borrowing

Rust's ownership system ensures memory safety without a garbage collector. In the linear search implementation, the array and target element are borrowed, meaning the function does not take ownership of them.

### Generics and Traits

The linear search function is generic, allowing it to work with any type that implements the `PartialEq` trait. This demonstrates Rust's powerful generics and trait system.

### Safe Memory Access

Rust ensures safe memory access by performing bounds checking on array accesses. The implementation uses safe indexing to prevent out-of-bounds access.

[Back to Table of Contents](#table-of-contents)

## Getting Started

To get started with this project, clone the repository and build it using Cargo:

```sh
git clone https://github.com/lkubicek1/rust_dsa.git
cd rust_dsa
cargo build
```

To run the unit tests, use:

```sh
cargo test
```

[Back to Table of Contents](#table-of-contents)

## License

This project is licensed under the Apache 2.0 License.

[Back to Table of Contents](#table-of-contents)
