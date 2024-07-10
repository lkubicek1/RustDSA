use crate::sorting::sort_tracker::SortTracker;

pub fn insertion_sort<T: PartialOrd>(arr: &mut [T], tracker: &mut SortTracker) {
    tracker.start_timer();
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && tracker.compare_lt(arr, j, j - 1) {
            tracker.swap(arr, j, j - 1);
            j -= 1;
        }
    }
    tracker.stop_timer();
}
