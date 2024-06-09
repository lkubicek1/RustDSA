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
│   ├── arrays
│   │   ├── mod.rs
│   │   ├── reverse_array.rs
│   │   ├── tests
│   │   │   ├── mod.rs
│   │   │   ├── reverse_array.rs
│   ├── search
│   │   ├── mod.rs
│   │   ├── linear_search.rs
│   │   ├── tests
│   │   │   ├── mod.rs
│   │   │   ├── linear_search.rs

    ... In progress
    
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

## Algorithms and Data Structures [:arrow_heading_up:](#table-of-contents)

### Arrays

The array module contains implementations of algorithms that operate on arrays.

#### Reverse Array

The reverse array algorithm reverses the elements of an array in place. The implementation is efficient and uses Rust's safety features.

```rust
pub fn reverse_array<T>(arr: &mut [T]) {
    let mut start = 0;
    let mut end = arr.len() - 1;
    while start < end {
        arr.swap(start, end);
        start += 1;
        end -= 1;
    }
}
```

#### Complexity Analysis

- **Time Complexity**: O(n) - The algorithm iterates through half of the array to reverse it.
- **Space Complexity**: O(1) - The algorithm uses a constant amount of extra space.
- **In-Place**: The algorithm reverses the array in place without using additional memory.

#### Test Cases

Tests are provided to ensure the correctness of the reverse array implementation.

``` shell 
cargo test reverse_array
```

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

``` shell
cargo test linear_search
```

## Rust Concepts [:arrow_heading_up:](#table-of-contents)

### Ownership and Borrowing

Rust's ownership system ensures memory safety without a garbage collector. In the linear search implementation, the array and target element are borrowed, meaning the function does not take ownership of them.

### Generics and Traits

The linear search function is generic, allowing it to work with any type that implements the `PartialEq` trait. This demonstrates Rust's powerful generics and trait system.

### Safe Memory Access

Rust ensures safe memory access by performing bounds checking on array accesses. The implementation uses safe indexing to prevent out-of-bounds access.

## Getting Started [:arrow_heading_up:](#table-of-contents)

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

## License [:arrow_heading_up:](#table-of-contents)

This project is licensed under the Apache 2.0 License.
