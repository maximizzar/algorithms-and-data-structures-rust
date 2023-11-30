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

impl BubbleSort  {
    fn sort(&mut self) {
        for _ in 0 .. self.elements.len().clone() {
            let mut sorted = true;
            for i in 0..self.elements.len().clone() - 1 {
                if self.elements[i] > self.elements[i + 1] {
                    sorted = false;
                    self.elements.swap(i,i + 1);
                }
            }
            if sorted {
                break;
            }
        }
    }
    pub fn get_runtime(&mut self) -> Option<Duration> {
        let start = std::time::Instant::now();
        BubbleSort::sort(self);
        let duration = start.elapsed();
        Some(duration)
    }
}
impl InsertionSort {
    pub fn sort(&mut self) {
        for i in 0..self.elements.len() - 2 {
            let mut i_next = i + 1;
            let n_current = self.elements[i_next];

            while i_next > 0 && n_current < self.elements[i_next - 1] {
                self.elements[i_next] = self.elements[i_next - 1];
                i_next -= 1;
            }
            self.elements[i_next] = n_current;
        }
    }
    pub fn get_runtime(&mut self) -> Option<Duration> {
        let start = std::time::Instant::now();
        InsertionSort::sort(self);
        let duration = start.elapsed();
        Some(duration)
    }
}

impl SelectionSort {
    pub fn sort(&mut self) {
        let highest_index = self.elements.len();
        let mut insertion_index: usize = 0;

        while insertion_index < highest_index {
            let mut min_position = insertion_index;

            for index in (insertion_index + 1)..highest_index {
                if self.elements[index] < self.elements[min_position] {
                    min_position = index;
                }
            }
            self.elements.swap(min_position,insertion_index);
            insertion_index += 1;
        }
    }
    pub fn get_runtime(&mut self) -> Option<Duration> {
        let start = std::time::Instant::now();
        SelectionSort::sort(self);
        let duration = start.elapsed();
        Some(duration)
    }
}

impl CountingSort {
    fn sort(&mut self, largest_value: usize) -> Vec<i64> {
        let mut c: Vec<i64> = vec![0; largest_value + 1];
        let mut output: Vec<i64> = vec![0; self.elements.len()];

        //count numbers in array
        for i in 0..self.elements.len() {
            c[self.elements[i] as usize] += 1;
        }

        //accumulate numbers in c
        for i in 1..largest_value {
            c[i] = c[i - 1] + c[i];
        }
        for i in 0..self.elements.len() {
            output[(c[self.elements[i] as usize] - 1) as usize] = self.elements[i];
            c[self.elements[i] as usize] -= 1;
        }
        return output;
    }
}
impl RadixSort {
    fn sort(&mut self, largest_value: usize) {
        let largest_power_of_ten = largest_value - largest_value % 10;
        let largest_power: i64 = (f64::log(largest_power_of_ten as f64, 10.0) as i64) + 1;

        for current_power in 0..largest_power as usize {
            let mut bucket_vector: Vec<Vec<i64>> = vec![vec![]; 10];

            // put input_vector in buckets
            for i in 0..self.elements.len() {
                let i_bucket_vector: usize = (self.elements[i] as f64 /
                    f64::powf(10.,current_power as f64)) as usize % 10;

                bucket_vector[i_bucket_vector].push(self.elements[i]);
            }

            //fill input_vector from buckets
            if current_power == (largest_power - 1) as usize {
                self.elements.copy_from_slice(&*bucket_vector[0]);
            } else {
                self.elements.clear();
                for c in 0..10 {
                    for r in 0..bucket_vector[c].len() {
                        self.elements.push(bucket_vector[c][r]);
                    }
                }
            }
        }
    }
}
