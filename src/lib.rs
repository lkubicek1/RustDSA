pub mod util {
    pub mod data_generation;
    pub mod reporting;
}

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
    pub mod graph_depth_first_search;

    #[cfg(test)]
    mod tests {
        mod linear_search_tests;
        mod binary_search_tests;
        mod graph_depth_first_search_tests;
    }
}

pub mod trees {
    pub mod heaps {
        pub mod heap_trait;
        pub mod heap;
        pub mod max_heap;
        pub mod min_heap;
        pub mod priority_queue;

        #[cfg(test)]
        mod tests {
            mod max_heap_tests;
            mod min_heap_tests;
            mod priority_queue_tests;
        }
    }
}

pub mod sorting {
    pub mod sort_tracker;

    pub mod insertion_sort;
    pub mod insertion_sort_with_tracking;

    pub mod merge_sort;
    pub mod merge_sort_with_tracking;

    pub mod heap_sort;
    pub mod heap_sort_with_tracking;

    pub mod quick_sort;
    pub mod quick_sort_mid;
    pub mod quick_sort_mod;
    pub mod quick_sort_with_tracking;
    pub mod quick_sort_mid_with_tracking;
    pub mod quick_sort_mod_with_tracking;

    pub mod graph_topological_sort;

    #[cfg(test)]
    mod tests {
        mod insertion_sort_tests;
        mod insertion_sort_with_tracking_tests;

        mod merge_sort_tests;
        mod merge_sort_with_tracking_tests;

        mod heap_sort_tests;
        mod heap_sort_with_tracking_tests;

        mod quick_sort_tests;
        mod quick_sort_mid_tests;
        mod quick_sort_mod_tests;
        mod quick_sort_with_tracking_tests;
        mod quick_sort_mid_with_tracking_tests;
        mod quick_sort_mod_with_tracking_tests;

        mod insertion_sort_benchmarks;
        mod merge_sort_benchmarks;
        mod heap_sort_benchmarks;
        mod quick_sort_benchmarks;
        mod quick_sort_mod_benchmarks;

        mod graph_topological_sort_tests;
    }
}
