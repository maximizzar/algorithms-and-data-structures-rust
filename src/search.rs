fn linear(array: &[i128], target: i128) -> usize {

    for i in 0..array.len() {
        if array[i] == target {
            return i;
        }
    }
    panic!("Couldn't find target value in array!")
}