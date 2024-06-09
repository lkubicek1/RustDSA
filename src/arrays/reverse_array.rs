pub fn reverse_array<T: Default>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len / 2 {
        let temp = std::mem::take(&mut arr[i]);
        arr[i] = std::mem::take(&mut arr[len - 1 - i]);
        arr[len - 1 - i] = temp;
    }
}