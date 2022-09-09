use std::fs;
// input output module used for taking input from user
use std::io;
// process module used for exiting program
use std::process;
// JSON module
use serde::{Serialize, Deserialize};

use std::fs::OpenOptions;
use std::io::prelude::*;

fn user_input(title: &str) -> String {
    println!("{}", title);
    let mut input = String::new();

    // make "input" variable = user input from terminal
    io::stdin().read_line(&mut input).expect("Failed to read line");

    input.trim().to_owned()
}

#[derive(Serialize, Deserialize, Debug)]
struct ContactInfo {
    name: String,
    phone: String,
    email: String,
}

// ContactBook methods
trait ContactOperations {
    fn post_to_file(&self);
    fn display_names(&self);
    fn new() -> Contacts;
    fn new_contact(&mut self);
    fn search_contacts(&self);
    fn edit_contact(&mut self);
    fn remove_contact(&mut self);
}

#[derive(Serialize, Deserialize, Debug)]
struct Contacts {
    contact_list: Vec<ContactInfo>
}

impl ContactOperations for Contacts {
    fn post_to_file(&self) {
    let serialized = serde_json::to_string(&self.contact_list).unwrap();
    let mut write = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open("contacts.txt")
            .expect("Unable to open file");

    write
        .write_all(serialized.as_bytes())
        .expect("Unable to write data");
    }

    fn new() -> Contacts {
        // construct Contacts with vector
        let contacts_file = fs::read_to_string("contacts.txt").unwrap();
        let contacts_vec = serde_json::from_str(&contacts_file).unwrap();
        Contacts { contact_list: contacts_vec }
    }

    fn new_contact(&mut self) {
        let name = user_input("NAME:");
        let phone = user_input("PHONE:");
        let email = user_input("EMAIL:");
        self.contact_list.push(ContactInfo { name, phone, email })
    }

    fn display_names(&self) {
        if self.contact_list.len() == 0 {
            println!("NO CONTACTS FOUND");
            return;
        }
        for contact in &self.contact_list {
            println!("{}", contact.name);
        }
    }

    fn search_contacts(&self) {
        let name = user_input("SEARCH NAME:");
        for contact in &self.contact_list {
            if name == contact.name {
                println!("NAME: {}", contact.name);
                println!("EMAIL: {}", contact.email);
                println!("PHONE: {}", contact.phone);
                return;
            }
        }
        print!("CONTACT NOT FOUND");
    }

    fn edit_contact(&mut self) {
        let name = user_input("NAME OF CONTACT: ");
        for contact in &mut self.contact_list.iter_mut() {
            if name == contact.name {
                let edit_name = user_input("NEW NAME: ");
                let edit_email = user_input("NEW EMAIL: ");
                let edit_phone = user_input("NEW PHONE: ");
                *contact = ContactInfo {name: edit_name, phone: edit_phone, email: edit_email};
                return;
            }
        }
        println!("CONTACT NOT FOUND");
    }

    fn remove_contact(&mut self) {
        let name = user_input("NAME OF CONTACT TO REMOVE: ");
        let length_before_deletion: usize = self.contact_list.len();
        self.contact_list.retain(|i| i.name != name);
        if length_before_deletion > self.contact_list.len() {
            println!("SUCCESSFULLY REMOVED");
        } else {
            println!("REMOVE FAILED");
        }
    }
}


fn display_menu(contacts: &mut Contacts) {
    println!(r#"

MAIN MENU
    1. Display Contact Names
    2. Search for Contact
    3. Edit Contact
    4. New Contact
    5. Remove Contact
    6. Exit
    "#);

    let mut input = String::new();

    // make "input" variable = user input from terminal
    io::stdin().read_line(&mut input).expect("Failed to read line");

    println!("\n");
    // remove trailing whitespace from input
    // (input from keyboard will always contain trailing newline character)

    match input.trim() {
        "1" => contacts.display_names(),
        "2" => contacts.search_contacts(),
        "3" => contacts.edit_contact(),
        "4" => contacts.new_contact(),
        "5" => contacts.remove_contact(),
        "6" => process::exit(1),
        _ => println!("Invalid choice"),
    }

    contacts.post_to_file();
}

fn main() {
    let mut contacts = Contacts::new();

    loop {
        display_menu(&mut contacts);
    }
}

