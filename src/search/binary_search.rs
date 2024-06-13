pub fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    let mut low: usize = 0;
    let mut high: usize = arr.len();

    while low < high {
        let mid = (high + low) / 2;
        if &arr[mid] < target {
            low = mid + 1;
        } else if &arr[mid] > target {
            high = mid;
        } else {
            return Some(mid);
        }
    }

    return None;
}