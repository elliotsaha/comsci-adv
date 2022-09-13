// input output module used for taking input from user
use std::io;

// helper function to get input from user and return String
pub fn user_input(title: &str) -> String {
    println!("{}", title);
    let mut input = String::new();

    // make "input" variable = user input from terminal
    io::stdin().read_line(&mut input).expect("Failed to read line");

    input.trim().to_owned()
}
