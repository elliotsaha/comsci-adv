use regex::Regex;
use std::fs::File;
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
