use std::time::Duration;

pub struct BubbleSort {
    pub elements: Vec<i64>,
}
pub struct InsertionSort {
    pub elements: Vec<i64>,
}
pub struct SelectionSort {
    pub elements: Vec<i64>,
}

pub struct CountingSort {
    pub elements: Vec<i64>,
}
pub struct RadixSort {
    pub elements: Vec<i64>,
}

impl BubbleSort()  {
    fn sort(&mut self) {
        for _element in self.elements {
            let mut sorted = true;
            for i in 0..self.len() - 1 {
                if self[i] > self[i + 1] {
                    sorted = false;
                    self.swap(i,i + 1);
                }
            }
            if sorted {
                break;
            }
        }
    }
    pub fn get_runtime(&mut self) -> Option<Duration> {
        let start = std::time::Instant::now();
        BubbleSort::sort(&mut self);
        let duration = start.elapsed();
        Some(duration)
    }
}
impl InsertionSort() {
    pub fn sort(&mut self) {
        for i in 0..self.len() - 2 {
            let mut i_next = i + 1;
            let n_current = self[i_next];

            while i_next > 0 && n_current < self[i_next - 1] {
                self[i_next] = self[i_next - 1];
                i_next -= 1;
            }
            self[i_next] = n_current;
        }
    }
    pub fn get_runtime(&mut self) -> Option<Duration> {
        let start = std::time::Instant::now();
        InsertionSort::sort(&mut self);
        let duration = start.elapsed();
        Some(duration)
    }
}

impl SelectionSort() {
    pub fn sort(&mut self) {
        let highest_index = self.len();
        let mut insertion_index: usize = 0;

        while insertion_index < highest_index {
            let mut min_position = insertion_index;

            for index in (insertion_index + 1)..highest_index {
                if self[index] < self[min_position] {
                    min_position = index;
                }
            }
            self.swap(min_position,insertion_index);
            insertion_index += 1;
        }
    }
    pub fn get_runtime(&mut self) -> Option<Duration> {
        let start = std::time::Instant::now();
        SelectionSort::sort(&mut self);
        let duration = start.elapsed();
        Some(duration)
    }
}

impl CountingSort() {
    fn sort(&mut self, largest_value: usize) -> self {
        let mut c: Vec<i64> = vec![0; largest_value + 1];
        let mut output: Vec<i64> = vec![0; self.len()];

        //count numbers in array
        for i in 0..self.len() {
            c[self[i] as usize] += 1;
        }

        //accumulate numbers in c
        for i in 1..largest_value {
            c[i] = c[i - 1] + c[i];
        }
        for i in 0..self.len() {
            output[(c[self[i] as usize] - 1) as usize] = self[i];
            c[self[i] as usize] -= 1;
        }
        return output;
    }
}
impl RadixSort() {
    fn sort(&mut self, largest_value: usize) {
        let largest_power_of_ten = largest_value - largest_value % 10;
        let largest_power: i64 = (f64::log(largest_power_of_ten as f64, 10.0) as i64) + 1;

        for current_power in 0..largest_power as usize {
            let mut bucket_vector: Vec<Vec<i64>> = vec![vec![]; 10];

            // put input_vector in buckets
            for i in 0..self.len() {
                let i_bucket_vector: usize = (self[i] as f64 /
                    f64::powf(10.,current_power as f64)) as usize % 10;

                bucket_vector[i_bucket_vector].push(self[i]);
            }

            //fill input_vector from buckets
            if current_power == (largest_power - 1) as usize {
                self.copy_from_slice(&*bucket_vector[0]);
            } else {
                self.clear();
                for c in 0..10 {
                    for r in 0..bucket_vector[c].len() {
                        self.push(bucket_vector[c][r]);
                    }
                }
            }
        }
    }
}
