// get user input helper
use crate::utils::user_input;
// file system module
use std::fs;
use std::fs::OpenOptions;
// JSON module
use serde::{Deserialize, Serialize};
// prelude functions
use std::io::prelude::*;

// derive(Serialize, Deserialize) is used for serde JSON parsing
#[derive(Serialize, Deserialize)]
struct ContactInfo {
    name: String,
    phone: String,
    email: String,
}

#[derive(Serialize, Deserialize)]
pub struct Contacts {
    contact_list: Vec<ContactInfo>,
}

// ContactBook methods
pub trait ContactOperations {
    fn post_to_file(&self);
    fn display_names(&self);
    fn new() -> Self;
    fn new_contact(&mut self);
    fn search_contacts(&self);
    fn edit_contact(&mut self);
    fn remove_contact(&mut self);
}

impl ContactOperations for Contacts {
    // writes vector as JSON formatted string to contactList.txt
    fn post_to_file(&self) {
        // get JSON formatted string from vector
        let serialized = serde_json::to_string(&self.contact_list).unwrap();
        // empty contactList file
        let mut write = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open("contactList.txt")
            .expect("Unable to open file");

        // write vector as string to contactList file
        write
            .write_all(serialized.as_bytes())
            .expect("Unable to write data");
    }

    // used as initializer for class (runs with constructor setting contact_list to parsed contents
    // of contactList.txt)
    fn new() -> Self {
        // reads contactList.txt contents as string
        let contacts_file = fs::read_to_string("contactList.txt").unwrap();
        // serde parses string into vector
        let contacts_vec = serde_json::from_str(&contacts_file).unwrap();

        Contacts {
            contact_list: contacts_vec,
        }
    }

    // create new contact
    fn new_contact(&mut self) {
        let name = user_input("NAME:");
        let email = user_input("EMAIL:");
        let phone = user_input("PHONE:");
        self.contact_list.push(ContactInfo { name, phone, email });
    }
    // display names of all contacts
    fn display_names(&self) {
        if self.contact_list.is_empty() {
            println!("NO CONTACTS FOUND");
            return;
        }

        // iterate through contacts vector and print name of each contact
        for contact in &self.contact_list {
            println!("{}", contact.name);
        }
    }

    fn search_contacts(&self) {
        let name = user_input("SEARCH NAME:");

        // iterate through vector and check if name matches
        for contact in &self.contact_list {
            if name == contact.name {
                // print all user info
                println!("Name: {}", contact.name);
                println!("Email: {}", contact.email);
                println!("Phone: {}", contact.phone);
                return;
            }
        }
        print!("CONTACT NOT FOUND");
    }

    fn edit_contact(&mut self) {
        let name = user_input("NAME OF CONTACT: ");

        // iterate through vector and check if name matches
        for contact in &mut self.contact_list {
            if name == contact.name {
                let edit_name = user_input("NEW NAME: ");
                let edit_email = user_input("NEW EMAIL: ");
                let edit_phone = user_input("NEW PHONE: ");

                // directly edit contact iterable
                *contact = ContactInfo {
                    name: edit_name,
                    phone: edit_phone,
                    email: edit_email,
                };

                return;
            }
        }
        println!("CONTACT NOT FOUND");
    }

    fn remove_contact(&mut self) {
        let name = user_input("NAME OF CONTACT TO REMOVE: ");

        let length_before_deletion: usize = self.contact_list.len();

        // iterates through array and only keeps contact if contact name is not
        // equal to name to remove
        self.contact_list.retain(|i| i.name != name);

        // if length of vector before deletion is the same as the current length of the vector,
        // then nothing has been removed
        if length_before_deletion > self.contact_list.len() {
            println!("SUCCESSFULLY REMOVED");
        } else {
            println!("CONTACT NOT FOUND");
        }
    }
}
