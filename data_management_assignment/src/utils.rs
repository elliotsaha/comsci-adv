use crate::music::{Song, SongController, SongOperations, SongTrait};
use crate::user::{UserController, UserOperations};
// used for pretty printing multiple lines of text
use indoc::indoc;
// process module used for exiting program
use std::process;
// table module used for pretty printing vectors of objects
use tabled::{Style, Table, Tabled};
// input output module used for taking input from user
use std::io::{self, Write};

// helper function to get input from user and return String
pub fn user_input(title: &str) -> String {
    // prepend and append new lines before printing title
    print!("\n{}\n", title);

    let mut input = String::new();

    io::stdout().flush().unwrap(); // manually flushes buffer (equivalent to \n)

    // make "input" variable = user input from terminal
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // trim output to ensure no extra spaces at the end or beginning
    input.trim().to_owned()
}

// helper function to create a table from a vector
pub fn table<T: Tabled>(vec: &Vec<T>) {
    if vec.len() > 0 {
        let mut table = Table::new(vec);
        table.with(Style::empty());
        println!("\n{table}\n");
    } else {
        // if the vector is empty, print error
        println!("No results");
    }
}

// helper enum to return either string type or number type
// derived traits are used to compare Or enums
#[derive(PartialEq, PartialOrd, Eq)]
pub enum Or {
    String(String),
    Num(i16),
}

// insertion sort function made specifically for sorting a vector of songs
// specific values are accessed by key and are compared on each iteration
pub fn insertion_sort(arr: &mut [Song], key: &str) {
    for i in 1..arr.len() {
        let curr_val = arr[i].clone();
        let mut curr_pos = i;

        while curr_pos > 0 && arr[curr_pos - 1].get(key) > curr_val.get(key) {
            arr[curr_pos] = arr[curr_pos - 1].clone();
            curr_pos -= 1;
        }

        arr[curr_pos] = curr_val;
    }
}

pub fn main_menu(song_controller: &mut SongController, user_controller: &mut UserController) {
    // if signed in, allow for user to add / display / remove from favourites
    if user_controller.auth_check(false) {
        let input = user_input(indoc! {"
            MAIN MENU
            1. Display All Songs
            2. Search Songs
            3. Sort Song List
            4. Display favourite songs
            5. Add to favourite songs
            6. Remove song from favourites
            7. Sign out
            8. Exit
        "});

        match input.as_str() {
            "1" => song_controller.display_all(),
            "2" => song_controller.search_menu(),
            "3" => song_controller.sort(),
            "4" => user_controller.display_favourites(),
            "5" => user_controller.add_favourite(),
            "6" => user_controller.remove_favourite(),
            "7" => user_controller.signout(),
            "8" => process::exit(1),
            _ => println!("Invalid choice"),
        }
    } else {
        // if not signed in, give options to create account or sign in instead
        let input = user_input(indoc! {"
            MAIN MENU
            1. Display All Songs
            2. Search Songs
            3. Sort Song List
            4. Sign in
            5. Create Account
            6. Exit
        "});

        match input.as_str() {
            "1" => song_controller.display_all(),
            "2" => song_controller.search_menu(),
            "3" => song_controller.sort(),
            "4" => user_controller.signin(),
            "5" => user_controller.register(),
            "6" => process::exit(1),
            _ => println!("Invalid choice"),
        }
    }
}
