mod search;

fn main() {
    const LARGEST_VALUE: usize = 10000;
    let master_vector: Vec<i64> = vec![7138, 5993, 1106, 4619, 1821, 7534, 7863];

    let search_target = 7863;
    {
        let vector = master_vector.clone();
        let start = std::time::Instant::now();
        search::linear_search(&vector, search_target);
        let duration = start.elapsed();
        println!("Time elapsed in linear_search() is: {:?}", duration);

    }
    {
        let mut vector: Vec<i64> = master_vector.clone();
        let start = std::time::Instant::now();
        search::sentinel_linear_search(&mut vector, search_target);
        let duration = start.elapsed();
        println!("Time elapsed in sentinel_linear_search() is: {:?}", duration);
    }

    println!("...");

    {
        let vector= master_vector.clone();
        let start = std::time::Instant::now();
        bubble_sort(vector);
        let duration = start.elapsed();
        println!("Time elapsed in bubble_sort() is: {:?}", duration);
    }
    {
        let vector= master_vector.clone();
        let start = std::time::Instant::now();
        counting_sort(vector.to_owned(),LARGEST_VALUE);
        let duration = start.elapsed();
        println!("Time elapsed in counting_sort() is: {:?}", duration);
    }
    {
        let mut vector = master_vector.clone();
        let start = std::time::Instant::now();
        insertion_sort(&mut vector);
        let duration = start.elapsed();
        println!("Time elapsed in insertion_sort() is: {:?}", duration);
    }
    {
        let mut vector = master_vector.clone();
        let start = std::time::Instant::now();
        radix_sort(&mut vector,LARGEST_VALUE);
        let duration = start.elapsed();
        println!("Time elapsed in radix_sort() is: {:?}", duration);
    }
    {
        let mut vector = master_vector.clone();
        let start = std::time::Instant::now();
        selection_sort(&mut vector);
        let duration = start.elapsed();
        println!("Time elapsed in selection_sort() is: {:?}", duration);
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

fn counting_sort(array: Vec<i64>, largest_value: usize) -> Vec<i64> {
    let mut c: Vec<i64> = vec![0; largest_value + 1];
    let mut output: Vec<i64> = vec![0; array.len()];

    //count numbers in array
    for i in 0..array.len() {
        c[array[i] as usize] += 1;
    }

    //accumulate numbers in c
    for i in 1..largest_value {
        c[i] = c[i - 1] + c[i];
    }
    for i in 0..array.len() {
        output[(c[array[i] as usize] - 1) as usize] = array[i];
        c[array[i] as usize] -= 1;
    }
    return output;
}

fn insertion_sort(vector: &mut Vec<i64>) -> Vec<i64> {
    for i in 0..vector.len() - 2 {
        let mut i_next = i + 1;
        let n_current = vector[i_next];

        while i_next > 0 && n_current < vector[i_next - 1] {
            vector[i_next] = vector[i_next - 1];
            i_next -= 1;
        }
        vector[i_next] = n_current;
    }
    return vector.to_owned();
}

fn radix_sort(vector: &mut Vec<i64>, largest_value: usize) -> Vec<i64> {
    let largest_power_of_ten = largest_value - largest_value % 10;
    let largest_power: i64 = (f64::log(largest_power_of_ten as f64, 10.0) as i64) + 1;

    for current_power in 0..largest_power as usize {
        let mut bucket_vector: Vec<Vec<i64>> = vec![vec![]; 10];

        // put input_vector in buckets
        for i in 0..vector.len() {
            let i_bucket_vector: usize = (vector[i] as f64 /
                f64::powf(10.,current_power as f64)) as usize % 10;

            bucket_vector[i_bucket_vector].push(vector[i]);
        }

        //fill input_vector from buckets
        if current_power == (largest_power - 1) as usize {
            vector.copy_from_slice(&*bucket_vector[0]);
        } else {
            vector.clear();
            for c in 0..10 {
                for r in 0..bucket_vector[c].len() {
                    vector.push(bucket_vector[c][r]);
                }
            }
        }
    }
    return vector.to_owned();
}

fn selection_sort(vector: &mut Vec<i64>) -> &mut Vec<i64> {
    let highest_index = vector.len();
    let mut insertion_index: usize = 0;

    while insertion_index < highest_index {
        let mut min_position = insertion_index;

        for index in (insertion_index + 1)..highest_index {
            if vector[index] < vector[min_position] {
                min_position = index;
            }
        }
        vector.swap(min_position,insertion_index);
        insertion_index += 1;
    }
    return vector;
}
