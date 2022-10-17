// process module used for exiting program
use std::process;
// import classes
use crate::spellcheck::{ SpellChecker, SpellCheckerOperations };
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
        "1" => SpellChecker::dictionary("linear"),
        "2" => SpellChecker::dictionary("binary"),
        "3" => SpellChecker::story("linear"),
        "4" => SpellChecker::story("binary"),
        "5" => process::exit(1),
        _ => println!("Invalid choice"),
    }
    
    println!("\n"); // add extra space
}
