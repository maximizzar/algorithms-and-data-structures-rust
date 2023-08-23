use std::ops::Index;

fn linear(array: &[i128], target: i128) -> usize {

    for i in 0..array.len() {
        if array[i] == target {
            return i;
        }
    }
    panic!("Couldn't find target value in array!");
}

fn sentinel_linear(vector: &mut Vec<i128>, target: i128) -> usize {
    let last_element: i128 = vector[vector.len() - 1];
    vector[vector.len() - 1] = target;
    let mut i: usize = 0;

    while vector[i] != target {
        i += 1;
    }
    vector[vector.len() - 1] = last_element;

    if i < vector.len() - 1 || last_element == target {
        return i;
    }
    panic!("Couldn't find target value in vector!");
}