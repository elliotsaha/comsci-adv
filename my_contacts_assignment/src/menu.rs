// process module used for exiting program
use std::process;
// import classes
use crate::contact::{ Contacts, ContactOperations };
// get user input helper
use crate::utils::user_input;

pub fn display_menu(contacts: &mut Contacts) {
    let input = user_input(r#"
MAIN MENU
    1. Display Contact Names
    2. Search for Contact
    3. Edit Contact
    4. New Contact
    5. Remove Contact
    6. Exit
"#);

    match input.as_str() {
        "1" => contacts.display_names(),
        "2" => contacts.search_contacts(),
        "3" => contacts.edit_contact(),
        "4" => contacts.new_contact(),
        "5" => contacts.remove_contact(),
        "6" => process::exit(1),
        _ => println!("Invalid choice"),
    }
    
    println!("\n"); // add extra space

    contacts.post_to_file();
}
