
fn main() {
    let mut array = [7,6,5,4,3,2,1,8];
    println!("{:?}", array);
    let lol = bubble_sort(&mut array);
    println!("{:?}", lol);
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