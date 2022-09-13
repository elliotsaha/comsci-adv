// process module used for exiting program
use std::process;
// import classes
use crate::nickname::{ NicknameGenerator, NicknameOperations };
// get user input helper function
use crate::utils::user_input;

pub fn display_menu(nickname_generator: &mut NicknameGenerator) {
    let input = user_input(r#"
MAIN MENU
    1. Change Name
    2. Display a Random Nickname
    3. Display All Nicknames
    4. Add a Nickname
    5. Remove a Nickname
    6. Exit
    "#);

    match input.as_str() {
        "1" => nickname_generator.prompt_name(),
        "2" => nickname_generator.display_random_nick(),
        "3" => nickname_generator.display_all_nicknames(),
        "4" => nickname_generator.add_nickname(),
        "5" => nickname_generator.remove_nickname(),
        "6" => process::exit(1),
        _ => println!("Invalid choice"),
    }

    println!("\n"); // add extra space
}
