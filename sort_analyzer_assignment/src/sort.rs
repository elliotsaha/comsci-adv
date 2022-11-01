// utility function imports
use crate::utils::file_to_vec;
// get time duration of function
use std::time::Instant;

pub struct FileSorter {}

pub trait SortMethods {
    // all sort methods returns time it takes to
    // sort through file
    fn bubble_sort(file: &str) -> f64;
    fn selection_sort(file: &str) -> f64;
    fn insertion_sort(file: &str) -> f64;
}

impl SortMethods for FileSorter {
    fn bubble_sort(file: &str) -> f64 {
        let mut data = file_to_vec(file);

        let now = Instant::now();

        for i in 0..data.len() - 1 {
            // subtracting by i to prevent unnecessary checks
            // on values that can be guaranteed to be swapped already
            // (largest values are naturally carried to end of array)
            for j in 0..data.len() - i - 1 {
                if data[j] > data[j + 1] {
                    data.swap(j, j + 1);
                }
            }
        }

        now.elapsed().as_secs_f64()
    }
    fn selection_sort(file: &str) -> f64 {
        let mut data = file_to_vec(file);

        let now = Instant::now();

        for i in 0..data.len() - 1 {
            // search for minimum
            let mut min = i;
            for j in (i + 1)..data.len() {
                if data[j] < data[min] {
                    // set current to minimum
                    min = j;
                }
            }
            data.swap(min, i)
        }

        now.elapsed().as_secs_f64()
    }
    fn insertion_sort(file: &str) -> f64 {
        let mut data = file_to_vec(file);

        let now = Instant::now();

        for i in 1..data.len() {
            let curr_val = data[i].clone();
            let mut curr_pos = i;

            while curr_pos > 0 && data[curr_pos - 1] > curr_val {
                data[curr_pos] = data[curr_pos - 1].clone();
                curr_pos -= 1;
            }

            data[curr_pos] = curr_val;
        }

        now.elapsed().as_secs_f64()
    }
}
