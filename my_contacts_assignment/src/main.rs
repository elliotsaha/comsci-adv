// input output module used for taking input from user
use std::io;
use std::io::Write;
// process module used for exiting program
use std::process;

fn user_input(title: &str) -> String {
    println!("{}", title);
    let mut input = String::new();

    // make "input" variable = user input from terminal
    io::stdin().read_line(&mut input).expect("Failed to read line");

    input.trim().to_owned()
}

struct ContactInfo {
    name: String,
    phone: String,
    email: String,
}

// ContactBook methods
trait ContactOperations {
    fn display_names(&self);
    fn new() -> Contacts;
    fn new_contact(&mut self);
    fn search_contacts(&self);
    fn edit_contact(&mut self);
    fn remove_contact(&mut self);
}


struct Contacts {
    contact_list: Vec<ContactInfo>
}

impl ContactOperations for Contacts {
    fn new() -> Contacts {
        // construct Contacts with vector
        Contacts { contact_list: Vec::new() }
    }

    fn new_contact(&mut self) {
        let name = user_input("NAME:");
        let phone = user_input("PHONE:");
        let email = user_input("EMAIL:");
        self.contact_list.push(ContactInfo { name, phone, email })
    }

    fn display_names(&self) {
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

fn main() {
    let mut contacts = Contacts::new();
    contacts.new_contact();
    contacts.remove_contact();
    contacts.display_names();
}

