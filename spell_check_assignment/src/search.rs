use crate::utils::{ file_to_vec, user_input, linear_search, binary_search };
use std::cmp::Ordering;

pub struct Search {}

pub trait SearchOperations {
    fn generate_vec(filename: &str) -> Vec<String>;
    fn input_linear(filename: &str);
    fn input_binary(filename: &str);
    fn compare_linear(filename: &str, compare_file: &str);
    fn compare_binary(filename: &str, compare_file: &str);
}

impl SearchOperations for Search {
    fn generate_vec(filename: &str) -> Vec<String> {
        // transform string into split vector
        let file_type = "txt";
        let dir = "data-files";
        file_to_vec(&format!("{dir}/{filename}.{file_type}"))
    }
    fn input_linear(filename: &str) {

        println!("Linear Search starting...");

        let split = Self::generate_vec(filename);

        let target = user_input("Please enter a word").to_lowercase();

        // loop through vector iteratively
        linear_search(split, |idx, i| {
            // check if target element is equal to iteration
            if i.to_lowercase() == target {
                print!("{} IS in vector at index {}", target, idx);
            }
        });
        print!("{} is NOT found in vector", target);
    }
    fn input_binary(filename: &str) {
        let target = user_input("Please enter a word").to_lowercase();

        println!("Linear Search starting...");

        let split = Self::generate_vec(filename);

        let mut start = 0;
        let mut end = split.len() - 1;
        
        // search all possible candidates for target in vector
        while start <= end {
            // get middle index of vector
            let middle = (((start + end) / 2) as f32).floor() as usize;
            match target.cmp(&split[middle].to_lowercase()) {
                Ordering::Equal => {
                    print!("{} IS in vector as index {}", target, middle);
                    return;
                }
                Ordering::Less => {
                    // search left half of vector
                    end = middle - 1;
                }
                Ordering::Greater => {
                    // search right half of vector
                    start = middle + 1;
                }
            }
        }
        print!("{} is NOT in vector", target);
    }
    
    fn compare_linear(filename: &str, compare_file: &str) {
    }

    fn compare_binary(filename: &str, compare_file: &str) {
        
    }
}
