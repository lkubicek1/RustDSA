pub fn quick_sort<T: PartialOrd>(arr: &mut [T]) {
    if !arr.is_empty() {
        quick_sort_recursive(arr, 0, arr.len() - 1)
    }
}

fn quick_sort_recursive<T: PartialOrd>(arr: &mut [T], low_index: usize, high_index: usize) {
    if high_index <= low_index {
        return;
    }

    let pivot_location: usize = partition(arr, low_index, high_index);

    quick_sort_recursive(arr, low_index, pivot_location);
    quick_sort_recursive(arr, pivot_location + 1, high_index);
}

fn partition<T: PartialOrd>(arr: &mut [T], low_index: usize, high_index: usize) -> usize {
    let pivot: usize = low_index;
    let mut leftwall: usize = low_index + 1;

    for i in (low_index+1)..=high_index {
        if arr[i] < arr[pivot] {
            arr.swap(i, leftwall);
            leftwall += 1;
        }
    }

    arr.swap(pivot, leftwall - 1);
    leftwall - 1
}