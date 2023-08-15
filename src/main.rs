
fn main() {
    let mut array = [7,6,5,4,3,2,1,8];
    println!("{:?}", array);
    println!("{:?}",bubble_sort(&mut array));
}
fn bubble_sort(array: &mut [i32]) -> &mut [i32] {
    for i in 0..array.len() {
        let mut sorted = true;
        for j in 0..array.len() - 1 {
            if array[j] < array[j] + 1 {
                sorted = false;
                let buffer = array[j];
                array[j] = array[j + 1];
                array[j + 1] = buffer;
            }
        }
        if sorted {
            break;
        }
    }
    return array;
}

/*
    public Integer[] bubbleSort(Integer[] arr) {
        for (int i = 0; i < arr.length; i++) {
            boolean sorted = true;
            for (int j = 0; j < arr.length - 1; j++) {
                if (arr[j] > arr[j + 1]) {
                    sorted = false;
                    int buffer = arr[j];
                    arr[j] = arr[j + 1];
                    arr[j + 1] = buffer;
                }
            }
            if (sorted) break;
        }
        return arr;
    }
 */