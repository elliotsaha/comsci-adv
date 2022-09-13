// get user input
use crate::utils::user_input;
// library to generate random numbers
use rand::Rng;

pub struct NicknameGenerator {
    first_name: String,
    last_name: String,
    nicknames: Vec<String>
}

// NicknameGenerator methods
pub trait NicknameOperations {
    fn new() -> Self;
    fn prompt_name(&mut self);
    fn display_random_nick(&self);
    fn display_all_nicknames(&self);
    fn add_nickname(&mut self);
    fn remove_nickname(&mut self);
}

impl NicknameOperations for NicknameGenerator {
    // used as initializer for class (runs with constructor setting first_name, last_name, and
    // default nicknames )
    fn new() -> NicknameGenerator {
        // transform each &str into String for nickname mutability
        let default_nicknames = vec!["The Crusher", "The Warrior", "The Hacker", "The Bulldog", "The Programmer"].iter().map(|&nick| nick.to_owned()).collect();
        NicknameGenerator { first_name: String::from(""), last_name: String::from(""), nicknames: default_nicknames }
    }
    
    // prompts user for their first and last name and mutate values
    fn prompt_name(&mut self) {
        let first_name = user_input("FIRST NAME:");
        let last_name = user_input("LAST NAME:");
        self.first_name = first_name;
        self.last_name = last_name;

        println!("Current name is {first_name} {last_name}", 
                 first_name = self.first_name, 
                 last_name = self.last_name);
    }
    
    // display a random nickname
    fn display_random_nick(&self) {
        // create random number generator
        let mut rng = rand::thread_rng();
        
        // generate random index to get random nickname
        let rand_idx = rng.gen_range(0..self.nicknames.len()); // [0, len)
                                                               //
        println!("RANDOM NICKNAME");
        println!("{first_name} “{random_nick}” {last_name}", 
                 first_name = self.first_name, 
                 random_nick = self.nicknames[rand_idx], 
                 last_name = self.last_name);
    }
    
    // get all nicknames from nicknames vector
    fn display_all_nicknames(&self) {
        println!("ALL NICKNAMES");
        // iterate through all nicknames and print
        for nick in &self.nicknames {
            println!("{first_name} “{nick}” {last_name}", 
                     first_name = self.first_name, 
                     nick = nick, 
                     last_name = self.last_name);
        }
    }
    
    // add new nickname
    fn add_nickname(&mut self) {
        let new_nick = user_input("ADD NICKNAME: ");

        // only push to nicknames vector if not included already
        if self.nicknames.contains(&new_nick) {
            println!("ALREADY A NICKNAME");
        } else {
            println!("SUCCESSFULLY ADDED NICKNAME");
            self.nicknames.push(new_nick);
        }
    }
    
    // remove nickname
    fn remove_nickname(&mut self) {
        let remove_nick = user_input("NICK TO REMOVE:");

        let length_before_deletion: usize = self.nicknames.len();


        // iterates through array and only keeps nickname if nickname is not 
        // equal to remove_nick
        self.nicknames.retain(|x| x != &remove_nick);


        // if length of vector before deletion is the same as the current length of the vector,
        // then nothing has been removed
        if length_before_deletion > self.nicknames.len() {
            println!("SUCCESSFULLY REMOVED");
        } else {
            println!("NICKNAME NOT FOUND");
        }
    }
}
