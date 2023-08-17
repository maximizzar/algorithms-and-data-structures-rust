
fn main() {
    let mut array = [7,6,5,4,3,2,1,8];

    let mut start = std::time::Instant::now();
    bubble_sort(&mut array);
    let mut duration = start.elapsed();
    println!("Time elapsed in bubble_sort() is: {:?}", duration);


    array = [1,5,6,7,4,4,7,8];

    start = std::time::Instant::now();
    insertion_sort(&mut array);
    duration = start.elapsed();
    println!("Time elapsed in insertion_sort() is: {:?}", duration);


    //lololo
}

fn bubble_sort(array: &mut [i64]) -> &[i64] {
    for _ in 0..array.len() {
        let mut sorted = true;
        for i in 0..array.len() - 1 {
           if array[i] > array[i + 1] {
               sorted = false;
               array.swap(i,i + 1);
           }
        }
        if sorted {
            break;
        }
    }
    return array;
}

fn insertion_sort(array: &mut [i64]) -> &[i64]{
    for j in 2..array.len() {
        let key = array[j];
        let mut i = j - 1;

        while i > 0 && array[i] > key {
            array[i + 1] = array[i];
            i -= 1;
        }
        array[i + 1] = key;
    }
    return array;
}