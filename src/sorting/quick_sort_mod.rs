use crate::sorting::insertion_sort::insertion_sort;

pub fn quick_sort<T: PartialOrd>(arr: &mut [T], cutoff: usize) {
    if arr.is_empty() {
        return;
    }

    quick_sort_recursive(arr, 0, arr.len() - 1, cutoff);
}

fn quick_sort_recursive<T: PartialOrd>(arr: &mut [T], low_index: usize, high_index: usize, cutoff: usize) {
    if high_index <= low_index {
        return;
    }

    if high_index - low_index + 1 <= cutoff {
        insertion_sort(&mut arr[low_index..=high_index]);
    } else {
        let pivot_location = partition(arr, low_index, high_index);

        if pivot_location > 0 {
            quick_sort_recursive(arr, low_index, pivot_location - 1, cutoff);
        }
        quick_sort_recursive(arr, pivot_location + 1, high_index, cutoff);
    }
}

fn partition<T: PartialOrd>(arr: &mut [T], low_index: usize, high_index: usize) -> usize {
    let midpoint = median_of_three(arr, low_index, high_index);

    arr.swap(midpoint, low_index);

    let pivot = low_index;
    let mut leftwall = low_index + 1;

    for i in leftwall..=high_index {
        if arr[i] < arr[pivot] {
            arr.swap(i, leftwall);
            leftwall += 1;
        }
    }

    arr.swap(pivot, leftwall - 1);
    leftwall - 1
}

fn median_of_three<T: PartialOrd>(arr: &mut [T], low_index: usize, high_index: usize) -> usize {
    let middle = (low_index + high_index) / 2;

    if arr[low_index] > arr[middle] {
        arr.swap(low_index, middle);
    }

    if arr[low_index] > arr[high_index] {
        arr.swap(low_index, high_index);
    }

    if arr[middle] > arr[high_index] {
        arr.swap(middle, high_index);
    }

    middle
}
