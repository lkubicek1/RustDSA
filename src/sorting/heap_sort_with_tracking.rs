use crate::sorting::sort_tracker::SortTracker;

pub fn heap_sort<T: PartialOrd+Default+PartialEq>(arr: &mut [T], tracker: &mut SortTracker) {
    tracker.start_timer();
    // Heapify array
    for i in (0..arr.len()/2).rev() {
        max_heap_percolate(i, arr, arr.len(), tracker)
    }

    for i in (0..arr.len()).rev() {
        arr.swap(0, i);
        max_heap_percolate(0, arr, i, tracker);
    }
    tracker.stop_timer();
}

fn max_heap_percolate<T: PartialOrd+Default+PartialEq>(mut node_index: usize, heap_arr: &mut [T], heap_size: usize, tracker: &mut SortTracker) {
    let mut child_index: usize = 2 * node_index + 1;

    while child_index < heap_size {
        let mut max_index: usize = node_index;

        for i in 0..2 {
            if i + child_index < heap_size && tracker.compare_lt(heap_arr, max_index, i + child_index) {
                max_index = i + child_index;
            }
        }

        if heap_arr[node_index].eq(&heap_arr[max_index]) {
            return;
        }

        if node_index != max_index {
            tracker.swap(heap_arr, node_index, max_index);
            node_index = max_index;
        }

        child_index = 2 * node_index + 1;
    }
}
