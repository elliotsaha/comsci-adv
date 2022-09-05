// input output module from standard library crate
use std::io;

fn display_menu() {
    println!(r#"
    MAIN MENU
        1. Display All Grades
        2. Display Honours
        3. Stats
        4. Randomize Grades
        5. Exit
    "#);

    let mut input = String::new();

    // make "input" variable = user input from terminal
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // remove trailing whitespace from input
    // (input from keyboard will always contain trailing newline character)
    match input.trim_end() {
        "1" => println!("YOU CHOSE 1"),
        "2" => println!("YOU CHOSE 2"),
        "3" => println!("YOU CHOSE 3"),
        "4" => println!("YOU CHOSE 4"),
        "5" => println!("YOU CHOSE 5"),
        _ => println!("Invalid choice"),
    }
}

fn main() {
    loop {
        display_menu();
    }
}
