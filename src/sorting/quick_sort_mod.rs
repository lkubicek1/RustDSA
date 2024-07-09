use crate::sorting::insertion_sort::insertion_sort;

pub fn quick_sort<T: PartialOrd>(arr: &mut [T], cutoff: usize) {
    if arr.is_empty() {
        return;
    }

    quick_sort_recursive(arr, 0, arr.len() - 1, cutoff)
}

fn quick_sort_recursive<T: PartialOrd>(arr: &mut [T], low_index: usize, high_index: usize, cutoff: usize) {
    if arr.is_empty() {
        return;
    }

    if high_index <= low_index {
        return;
    }

    if arr.len() <= cutoff {
        insertion_sort(arr);
    } else {
        let pivot: usize = median_of_three(arr);
        let high_index: usize = arr.len() - 1;
        arr.swap(pivot, high_index);

        let p: usize = partition(arr, 0, high_index);
        if p > 0 {
            quick_sort_recursive(arr, 0, p - 1, cutoff);
        }
        quick_sort_recursive(arr, p + 1, high_index, cutoff);
    }
}

fn partition<T: PartialOrd>(arr: &mut [T], low_index: usize, high_index: usize) -> usize {
    let pivot: usize = high_index;
    let mut leftwall: usize = low_index;

    for i in low_index..high_index {
        if arr[i] < arr[pivot] {
            arr.swap(i, leftwall);
            leftwall += 1;
        }
    }

    arr.swap(leftwall, high_index);
    leftwall
}

fn median_of_three<T: PartialOrd>(arr: &mut [T]) -> usize {
    let first: usize = 0;
    let middle: usize = arr.len() / 2;
    let last: usize = arr.len() - 1;

    if arr[first] > arr[middle] {
        arr.swap(first, middle);
    }

    if arr[first] > arr[last] {
        arr.swap(first, last);
    }

    if arr[middle] > arr[last] {
        arr.swap(middle, last);
    }

    middle
}