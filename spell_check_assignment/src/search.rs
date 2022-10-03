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
        
        let mut found = false;

        // loop through vector iteratively
        linear_search(split, &target, |idx| {
                print!("{} IS in vector at index {}", target, idx);
                found = true;
            });

        if !found {
            print!("{} is NOT found in vector", target);
        }
    }
    fn input_binary(filename: &str) {
        let target = user_input("Please enter a word").to_lowercase();

        println!("Linear Search starting...");

        let split = Self::generate_vec(filename);
        
        let mut found = false;

        binary_search(split, &target, |idx| {
            print!("{} IS in vector at index {}", target, idx);
            found = true;
        });

        print!("{} is NOT in vector", target);
    }
    
    fn compare_linear(filename: &str, compare_file: &str) {
    }

    fn compare_binary(filename: &str, compare_file: &str) {
        
    }
}
