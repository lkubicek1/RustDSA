use crate::sorting::sort_tracker::SortTracker;

pub fn quick_sort<T: PartialOrd>(arr: &mut [T], tracker: &mut SortTracker) {
    tracker.start_timer();
    if !arr.is_empty() {
        quick_sort_recursive(arr, 0, arr.len() - 1, tracker);
    }
    tracker.stop_timer();
}

fn quick_sort_recursive<T: PartialOrd>(arr: &mut [T], low_index: usize, high_index: usize, tracker: &mut SortTracker) {
    if high_index <= low_index {
        return;
    }

    let pivot_location: usize = partition(arr, low_index, high_index, tracker);

    quick_sort_recursive(arr, low_index, pivot_location, tracker);
    quick_sort_recursive(arr, pivot_location + 1, high_index, tracker);
}

fn partition<T: PartialOrd>(arr: &mut [T], low_index: usize, high_index: usize, tracker: &mut SortTracker) -> usize {
    let pivot: usize = low_index;
    let mut leftwall: usize = low_index + 1;

    for i in (low_index+1)..=high_index {
        if tracker.compare_lt(arr, i, pivot) {
            tracker.swap(arr, i, leftwall);
            leftwall += 1;
        }
    }

    tracker.swap(arr, pivot, leftwall - 1);
    leftwall - 1
}