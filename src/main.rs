fn main() {
    let mut master_array = vec![5,6,4,3,6,8,5,5,8,4,2,6,9,4,3,1,0,1,0,0,4,5,8,7,3,2,4,5,3,2,1,5];
    {
        let array = master_array.clone();
        let start = std::time::Instant::now();
        bubble_sort(array);
        let duration = start.elapsed();
        println!("Time elapsed in bubble_sort() is: {:?}", duration);
    }
    {
        let array = master_array.clone();

        let start = std::time::Instant::now();
        counting_sort(array.to_owned(), 10);
        let duration = start.elapsed();
        println!("Time elapsed in counting_sort() is: {:?}", duration);
    }
    {
        let array = master_array.clone();
        let start = std::time::Instant::now();
        insertion_sort(array);
        let duration = start.elapsed();
        println!("Time elapsed in insertion_sort() is: {:?}", duration);
    }
}

fn bubble_sort(mut array: Vec<i64>) -> Vec<i64> {
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

fn counting_sort(array: Vec<i64>, biggest_number: usize) -> Vec<i64> {
    let mut c: Vec<i64> = vec![0; biggest_number + 1];
    let mut output: Vec<i64> = vec![0; array.len()];

    //count numbers in array
    for i in 0..array.len() {
        c[array[i] as usize] += 1;
    }

    //accumulate numbers in c
    for i in 1..biggest_number {
        c[i] = c[i - 1] + c[i];
    }
    for i in 0..array.len() {
        output[(c[array[i] as usize] - 1) as usize] = array[i];
        c[array[i] as usize] -= 1;
    }
    return output;
}
fn insertion_sort(mut array: Vec<i64>) -> Vec<i64> {
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