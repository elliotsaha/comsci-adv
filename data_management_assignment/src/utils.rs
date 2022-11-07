use std::cmp::Ordering;
// input output module used for taking input from user
use std::io::{self, Write};

// helper function to get input from user and return String
pub fn user_input(title: &str) -> String {
    print!("{}", title);

    let mut input = String::new();

    io::stdout().flush().unwrap(); // manually flushes buffer (equivalent to \n)

    // make "input" variable = user input from terminal
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_owned()
}

pub fn binary_search<T>(list: &[T], target: T) -> isize
// each generic element must have Ord trait for match comparison
where
    T: std::cmp::Ord,
{
    let mut start = 0;
    let mut end = list.len() - 1;

    // search all possible candidates for target in vector
    while start <= end {
        // get middle index of vector
        let middle = (((start + end) / 2) as f32).floor() as usize;

        match target.cmp(&list[middle]) {
            Ordering::Equal => {
                return middle as isize;
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
    // return -1 if not found
    -1
}
