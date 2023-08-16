
fn main() {
    let mut array = [7,6,5,4,3,2,1,8];
    println!("{:?}", array);
    println!("{:?}", bubble_sort(&mut array));
}
fn bubble_sort(array: &mut [i64]) {
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
}