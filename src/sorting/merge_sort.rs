pub struct MergeSort { }

impl MergeSort {
    pub fn new() -> Self {
        MergeSort { }
    }

    pub fn sort<T: PartialOrd+Default>(&self, arr: &mut [T]) {
        if !arr.is_empty() {
            self.merge_sort(arr, 0, arr.len() - 1);
        }
    }

    fn merge_sort<T: PartialOrd+Default>(&self, arr: &mut [T], start_index: usize, end_index: usize) {
        if start_index < end_index {
            let mid: usize = (start_index + end_index) / 2;

            self.merge_sort(arr, start_index, mid);
            self.merge_sort(arr, mid + 1, end_index);

            self.merge(arr, start_index, mid, end_index);
        }
    }

    fn merge<T: PartialOrd+Default>(&self, arr: &mut [T], left_first: usize, left_last: usize, right_last: usize) {
        let merged_size: usize = right_last - left_first + 1;
        let mut merged_entries: Vec<T> = Vec::with_capacity(merged_size);
        let mut left_position: usize = left_first;
        let mut right_position: usize = left_last + 1;

        while left_position <= left_last && right_position <= right_last {
            if arr[left_position] <= arr[right_position] {
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
}