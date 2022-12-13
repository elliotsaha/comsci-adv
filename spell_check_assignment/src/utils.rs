use regex::Regex;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::prelude::*;

pub fn file_to_vec(filename: &str) -> Vec<String> {
    let file_type = "txt";
    let dir = "data-files";
    let path = &format!("{dir}/{filename}.{file_type}");

    // read file and parse to string
    let mut file = File::open(path).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read the file");
    // regex pattern to seperate by spaces
    let re = Regex::new(r"\s+").unwrap();
    // trim leading and trailing whitespaces and split by regex
    let split_vec = re.split(contents.trim()).map(String::from).collect();

    split_vec
}

// helper function to get input from user and return String
pub fn user_input(title: &str) -> String {
    // prompt for user
    println!("{}", title);

    let mut input = String::new();

    // make "input" variable = user input from terminal
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_owned()
}

// capitalizes first letter in a string
pub fn capitalize(s: &str) -> String {
    s[0..1].to_uppercase() + &s[1..]
}

pub fn linear_search<F: FnMut(usize)>(array: &[String], target: &str, mut f: F) {
    for idx in 0..array.len() {
        if array[idx] == target {
            return f(idx);
        }
    }
}

pub fn binary_search<F: FnMut(usize)>(vector: &[String], target: &str, mut f: F) {
    let mut start = 0;
    let mut end = vector.len() - 1;

    // search all possible candidates for target in vector
    while start <= end {
        // get middle index of vector
        let middle = (start + end) / 2;

        match target.cmp(&vector[middle]) {
            Ordering::Equal => {
                return f(middle);
            }
            Ordering::Less => {
                // make sure index cannot be less than 0
                if middle == 0 {
                    return;
                };
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
