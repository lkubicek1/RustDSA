use crate::sorting::insertion_sort_with_tracking::insertion_sort;
use crate::sorting::sort_tracker::SortTracker;

pub fn quick_sort<T: PartialOrd>(arr: &mut [T], cutoff: usize, tracker: &mut SortTracker) {
    if arr.is_empty() {
        return;
    }

    quick_sort_recursive(arr, 0, arr.len() - 1, cutoff, tracker)
}

fn quick_sort_recursive<T: PartialOrd>(arr: &mut [T], low_index: usize, high_index: usize, cutoff: usize, tracker: &mut SortTracker) {
    if arr.is_empty() {
        return;
    }

    if high_index <= low_index {
        return;
    }

    if arr.len() <= cutoff {
        insertion_sort(arr, tracker);
    } else {
        let pivot: usize = median_of_three(arr, tracker);
        let high_index: usize = arr.len() - 1;
        tracker.swap(arr, pivot, high_index);

        let p: usize = partition(arr, 0, high_index, tracker);
        if p > 0 {
            quick_sort_recursive(arr, 0, p - 1, cutoff, tracker);
        }
        quick_sort_recursive(arr, p + 1, high_index, cutoff, tracker);
    }
}

fn partition<T: PartialOrd>(arr: &mut [T], low_index: usize, high_index: usize, tracker: &mut SortTracker) -> usize {
    let pivot: usize = high_index;
    let mut leftwall: usize = low_index;

    for i in low_index..high_index {
        if tracker.compare_lt(arr, i, pivot) {
            tracker.swap(arr, i, leftwall);
            leftwall += 1;
        }
    }

    tracker.swap(arr, leftwall, high_index);
    leftwall
}

fn median_of_three<T: PartialOrd>(arr: &mut [T], tracker: &mut SortTracker) -> usize {
    let first: usize = 0;
    let middle: usize = arr.len() / 2;
    let last: usize = arr.len() - 1;

    if tracker.compare_gt(arr, first, middle) {
        tracker.swap(arr, first, middle);
    }

    if tracker.compare_gt(arr, first, last) {
        tracker.swap(arr, first, last);
    }

    if tracker.compare_gt(arr, middle, last) {
        tracker.swap(arr, middle, last);
    }

    middle
}