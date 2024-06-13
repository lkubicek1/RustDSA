pub mod arrays {
    pub mod reverse_array;

    #[cfg(test)]
    mod tests {
        mod reverse_array_tests;
    }
}

pub mod search {
    pub mod linear_search;
    pub mod binary_search;

    #[cfg(test)]
    mod tests {
        mod linear_search_tests;
        mod binary_search_tests;
    }
}
