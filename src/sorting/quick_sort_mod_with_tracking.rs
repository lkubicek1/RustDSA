use crate::sorting::insertion_sort_with_tracking::insertion_sort;
use crate::sorting::sort_tracker::SortTracker;

pub fn quick_sort<T: PartialOrd>(arr: &mut [T], cutoff: usize, tracker: &mut SortTracker) {
    tracker.start_timer();
    if arr.is_empty() {
        return;
    }

    quick_sort_recursive(arr, 0, arr.len() - 1, cutoff, tracker);
    tracker.stop_timer();
}

fn quick_sort_recursive<T: PartialOrd>(arr: &mut [T], low_index: usize, high_index: usize, cutoff: usize, tracker: &mut SortTracker) {
    if high_index <= low_index {
        return;
    }

    if high_index - low_index + 1 <= cutoff {
        insertion_sort(&mut arr[low_index..=high_index], tracker);
    } else {
        let pivot_location = partition(arr, low_index, high_index, tracker);

        if pivot_location > 0 {
            quick_sort_recursive(arr, low_index, pivot_location - 1, cutoff, tracker);
        }
        quick_sort_recursive(arr, pivot_location + 1, high_index, cutoff, tracker);
    }
}

fn partition<T: PartialOrd>(arr: &mut [T], low_index: usize, high_index: usize, tracker: &mut SortTracker) -> usize {
    let midpoint = median_of_three(arr, low_index, high_index, tracker);

    tracker.swap(arr, midpoint, low_index);

    let pivot = low_index;
    let mut leftwall = low_index + 1;

    for i in leftwall..=high_index {
        if tracker.compare_lt(arr, i, pivot) {
            tracker.swap(arr, i, leftwall);
            leftwall += 1;
        }
    }

    tracker.swap(arr, pivot, leftwall - 1);
    leftwall - 1
}

fn median_of_three<T: PartialOrd>(arr: &mut [T], low_index: usize, high_index: usize, tracker: &mut SortTracker) -> usize {
    let middle = (low_index + high_index) / 2;

    if tracker.compare_gt(arr, low_index, middle) {
        tracker.swap(arr, low_index, middle);
    }

    if tracker.compare_gt(arr, low_index, high_index) {
        tracker.swap(arr, low_index, high_index);
    }

    if tracker.compare_gt(arr, middle, high_index) {
        tracker.swap(arr, middle, high_index);
    }

    middle
}
