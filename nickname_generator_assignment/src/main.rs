// input output module used for taking input from user
use std::io;
// process module used for exiting program
use std::process;
// library to generate random numbers
use rand::Rng;

fn user_input(title: &str) -> String {
    println!("{}", title);
    let mut input = String::new();

    // make "input" variable = user input from terminal
    io::stdin().read_line(&mut input).expect("Failed to read line");

    input.trim().to_owned()
}

struct NicknameGenerator {
    first_name: String,
    last_name: String,
    nicknames: Vec<String>
}

trait NicknameOperations {
    fn new() -> Self;
    fn prompt_name(&mut self);
    fn display_random_nick(&self);
    fn display_all_nicknames(&self);
    fn add_nickname(&mut self);
    fn remove_nickname(&mut self);
}

impl NicknameOperations for NicknameGenerator {
    fn new() -> NicknameGenerator {
        let default_nicknames: Vec<String> = ["The Crusher", "The Warrior", "The Hacker", "The Bulldog", "The Programmer"].map(String::from).to_vec();
        NicknameGenerator { first_name: "".to_owned(), last_name: "".to_owned(), nicknames: default_nicknames }
    }

    fn prompt_name(&mut self) {
        let first_name = user_input("FIRST NAME:");
        let last_name = user_input("LAST NAME:");
        self.first_name = first_name;
        self.last_name = last_name;
        println!("Current name is {} {}", self.first_name, self.last_name);
    }

    fn display_random_nick(&self) {
        let mut rng = rand::thread_rng();
        let rand_idx = rng.gen_range(0..self.nicknames.len()); // max idx is len - 1
        println!("RANDOM NICKNAME");
        println!("{} “{}” {}", self.first_name, self.nicknames[rand_idx], self.last_name);
    }

    fn display_all_nicknames(&self) {
        println!("Displaying all nicknames");
        for nick in &self.nicknames {
            println!("{} “{}” {}", self.first_name, nick, self.last_name);
        }
    }

    fn add_nickname(&mut self) {
        let new_nick = user_input("ADD NICKNAME: ");
        if self.nicknames.contains(&new_nick) {
            println!("already a nickname");
        } else {
            println!("added nickname");
            self.nicknames.push(new_nick);
        }
    }

    fn remove_nickname(&mut self) {
        let remove_nick = user_input("NICK TO REMOVE:");
        let length_before_deletion: usize = self.nicknames.len();
        self.nicknames.retain(|x| x != &remove_nick);
        if length_before_deletion > self.nicknames.len() {
            println!("SUCCESSFULLY REMOVED");
        } else {
            println!("REMOVE FAILED");
        }
    }
}

fn display_menu(nickname_generator: &mut NicknameGenerator) {
    println!(r#"
MAIN MENU
    1. Change Name
    2. Display a Random Nickname
    3. Display All Nicknames
    4. Add a Nickname
    5. Remove a Nickname
    6. Exit
    "#);

    let mut input = String::new();

    // make "input" variable = user input from terminal
    io::stdin().read_line(&mut input).expect("Failed to read line");

    println!("\n");
    // remove trailing whitespace from input
    // (input from keyboard will always contain trailing newline character)
    match input.trim() {
        "1" => nickname_generator.prompt_name(),
        "2" => nickname_generator.display_random_nick(),
        "3" => nickname_generator.display_all_nicknames(),
        "4" => nickname_generator.add_nickname(),
        "5" => nickname_generator.remove_nickname(),
        "6" => process::exit(1),
        _ => println!("Invalid choice"),
    }
}

fn main() {
    let mut nickname_generator = NicknameGenerator::new();
    
    nickname_generator.prompt_name();

    loop {
        display_menu(&mut nickname_generator);
    }
}
