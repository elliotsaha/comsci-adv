// input output module used for taking input from user
use std::io;
// process module used for exiting program
use std::process;
// import classes
use crate::{ GradeBook, GradeOperations };

pub fn display_menu(grade_book: &mut GradeBook) {
    // displays main menu
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

    println!("\n");
    // remove trailing whitespace from input
    // (input from keyboard will always contain trailing newline character)
    match input.trim() {
        "1" => grade_book.show_all(),
        "2" => grade_book.honours(),
        "3" => grade_book.stats(),
        "4" => grade_book.randomize(),
        "5" => process::exit(1),
        _ => println!("Invalid choice"),
    }
}

