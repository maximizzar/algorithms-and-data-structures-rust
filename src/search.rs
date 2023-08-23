pub fn linear_search(array: &Vec<i64>, search_target: i64) -> Option<usize> {

    for i in 0..array.len() {
        if array[i] == search_target {
            return Some(i);
        }
    }
    None
}
pub fn sentinel_linear_search(vector: &mut Vec<i64>, search_target: i64) -> Option<usize> {
    let last_element: i64 = vector[vector.len() - 1];
    let last_index: usize = vector.len().clone() - 1;

    vector[last_index] = search_target;
    let mut i: usize = 0;

    while vector[i] != search_target {
        i += 1;
    }

    vector[last_index] = last_element;

    if i < vector.len() - 1 || last_element == search_target {
        return Some(i);
    }
    None
}