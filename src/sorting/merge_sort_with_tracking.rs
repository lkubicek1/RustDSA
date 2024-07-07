use crate::sorting::sort_tracker::SortTracker;

pub fn merge_sort<T: PartialOrd + Default>(arr: &mut [T], tracker: &mut SortTracker) {
    if !arr.is_empty() {
        merge_sort_recursive(arr, 0, arr.len() - 1, tracker);
    }
}

fn merge_sort_recursive<T: PartialOrd + Default>(arr: &mut [T], start_index: usize, end_index: usize, tracker: &mut SortTracker) {
    if start_index < end_index {
        let mid: usize = (start_index + end_index) / 2;

        merge_sort_recursive(arr, start_index, mid, tracker);
        merge_sort_recursive(arr, mid + 1, end_index, tracker);

        merge(arr, start_index, mid, end_index, tracker);
    }
}

fn merge<T: PartialOrd + Default>(arr: &mut [T], left_first: usize, left_last: usize, right_last: usize, tracker: &mut SortTracker) {
    let merged_size: usize = right_last - left_first + 1;
    let mut merged_entries: Vec<T> = Vec::with_capacity(merged_size);
    let mut left_position: usize = left_first;
    let mut right_position: usize = left_last + 1;

    while left_position <= left_last && right_position <= right_last {
        if tracker.compare_lte(arr, left_position, right_position) {
            merged_entries.push(std::mem::take(&mut arr[left_position]));
            left_position += 1;
        } else {
            merged_entries.push(std::mem::take(&mut arr[right_position]));
            right_position += 1;
        }
    }

    while left_position <= left_last {
        merged_entries.push(std::mem::take(&mut arr[left_position]));
        left_position += 1;
    }

    while right_position <= right_last {
        merged_entries.push(std::mem::take(&mut arr[right_position]));
        right_position += 1;
    }

    for merge_position in 0..merged_size {
        arr[left_first + merge_position] = std::mem::take(&mut merged_entries[merge_position]);
    }
}
