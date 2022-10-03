use std::io;
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;
use std::cmp::Ordering;

pub fn file_to_vec(filename: &str) -> Vec<String> {
    // read file and parse to string
    let mut file = File::open(filename).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    // split string by regex pattern and transform into vector
    let re = Regex::new(r"\s+").unwrap();
    let split_vec = re.split(&contents).map(String::from).collect();
    split_vec 
}

// helper function to get input from user and return String
pub fn user_input(title: &str) -> String {
    println!("{}", title);
    let mut input = String::new();

    // make "input" variable = user input from terminal
    io::stdin().read_line(&mut input).expect("Failed to read line");

    input.trim().to_owned()
}

pub fn linear_search<F: FnMut(usize)>(vector: Vec<String>, target: &str, mut f: F) {
   for (idx, i) in vector.iter().enumerate() {
       if i.to_lowercase() == target {
           f(idx);
           return;
       }
   } 
}

pub fn binary_search<F: FnMut(usize)>(vector:Vec<String>, target: &str, f: F) {
    let mut start = 0;
    let mut end = vector.len() - 1;
    
    // search all possible candidates for target in vector
    while start <= end {
        // get middle index of vector
        let middle = (((start + end) / 2) as f32).floor() as usize;
        match target.cmp(&vector[middle].to_lowercase()) {
            Ordering::Equal => {
                f(middle);
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
}
