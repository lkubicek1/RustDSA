use std::cmp::PartialOrd;

pub struct SortTracker {
    comparison_count: usize,
    swap_count: usize,
}

impl SortTracker {
    pub fn new() -> Self {
        SortTracker {
            comparison_count: 0,
            swap_count: 0,
        }
    }

    pub fn get_comparison_count(&self) -> usize {
        self.comparison_count
    }

    pub fn get_swap_count(&self) -> usize {
        self.swap_count
    }

    pub fn compare_lt<T: PartialOrd>(&mut self, array: &[T], index1: usize, index2: usize) -> bool {
        self.comparison_count += 1;
        array[index1] < array[index2]
    }

    pub fn compare_lte<T: PartialOrd>(&mut self, array: &[T], index1: usize, index2: usize) -> bool {
        self.comparison_count += 1;
        array[index1] <= array[index2]
    }

    pub fn swap<T>(&mut self, array: &mut [T], index1: usize, index2: usize) {
        self.swap_count += 1;
        array.swap(index1, index2);
    }
}