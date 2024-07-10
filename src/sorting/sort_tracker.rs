use std::cmp::PartialOrd;
use std::time::{Instant, Duration};

pub struct SortTracker {
    comparison_count: usize,
    swap_count: usize,
    start_time: Option<Instant>,
    duration: Duration,
    lock_timer: bool,
}

impl SortTracker {
    pub fn new() -> Self {
        SortTracker {
            comparison_count: 0,
            swap_count: 0,
            start_time: None,
            duration: Duration::new(0, 0),
            lock_timer: false,
        }
    }

    pub fn reset(&mut self) {
        self.comparison_count = 0;
        self.swap_count = 0;
        self.start_time = None;
        self.duration = Duration::new(0, 0);
        self.lock_timer = false;
    }

    pub fn lock_timer(&mut self) {
        self.lock_timer = true;
    }

    pub fn unlock_timer(&mut self) {
        self.lock_timer = false;
    }

    pub fn get_comparison_count(&self) -> usize {
        self.comparison_count
    }

    pub fn get_swap_count(&self) -> usize {
        self.swap_count
    }

    pub fn compare_gt<T: PartialOrd>(&mut self, array: &[T], index1: usize, index2: usize) -> bool {
        self.comparison_count += 1;
        array[index1] > array[index2]
    }

    pub fn compare_gte<T: PartialOrd>(&mut self, array: &[T], index1: usize, index2: usize) -> bool {
        self.comparison_count += 1;
        array[index1] >= array[index2]
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

    pub fn start_timer(&mut self) {
        if self.lock_timer {
            return;
        }
        self.start_time = Some(Instant::now());
    }

    pub fn stop_timer(&mut self) {
        if self.lock_timer {
            return;
        }
        if let Some(start) = self.start_time {
            self.duration = start.elapsed();
        }
    }

    pub fn get_duration(&self) -> Duration {
        self.duration
    }
}