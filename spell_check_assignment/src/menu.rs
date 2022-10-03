// process module used for exiting program
use std::process;
// import classes
use crate::search::{ Search, SearchOperations };
// get user input helper function
use crate::utils::user_input;

pub fn display_menu() {
    let input = user_input(r#"
MAIN MENU
    1: Spell Check a Word (Linear Search)
    2: Spell Check a Word (Binary Search)
    3: Spell Check Alice In Wonderland (Linear Search)
    4: Spell Check Alice In Wonderland (Binary Search)
    5: Exit
"#);

    match input.as_str() {
        "1" => Search::input_linear("dictionary"),
        "2" => Search::input_binary("dictionary"),
        "3" => Search::input_linear("AliceInWonderLand"),
        "4" => Search::input_binary("AliceInWonderLand"),
        "5" => process::exit(1),
        _ => println!("Invalid choice"),
    }
    
    println!("\n"); // add extra space
}
