// utility functions
use crate::utils::{table, user_input, Or};
// Song struct and operations
use crate::music::{Song, SongController, SongOperations, SongTrait};
// file system module
use std::fs::{self, OpenOptions};
// JSON module
use serde::{Deserialize, Serialize};
// prelude functions
use std::io::prelude::*;
// argon2 is a password hashing algorithm
use argon2::{self, Config};
// salt generation
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

// properties that are made for each user
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
    pub favourites: Vec<Song>,
}

// UserController contains main vector that holds User structs as well as a copy of the Song
// Controller and a saved token that identifies the current user signed in
#[derive(Serialize, Deserialize, Clone)]
pub struct UserController {
    user_vec: Vec<User>,
    token: String, // when user signs in, token is assigned to user id
    song_controller: SongController,
}

pub trait UserOperations {
    fn new(song_controller: SongController) -> Self;
    // post to db file
    fn save(&self);
    // creates new user
    fn register(&mut self);
    // sign in user
    fn signin(&mut self);
    // signs out user
    fn signout(&mut self);
    // returns true if token != empty and prints error if it does (if warning is set to true)
    fn auth_check(&self, warning: bool) -> bool;
    // gets data from the user that is currently signed in
    fn get_user(&mut self) -> Option<&mut User>;
    // displays the user's favourite songs
    fn display_favourites(&mut self);
    // prompts menu to add favourite song
    fn add_favourite(&mut self);
    // prompts menu to remove favourite song
    fn remove_favourite(&mut self);
}

impl UserOperations for UserController {
    fn new(song_controller: SongController) -> Self {
        // reads db.txt contents as string
        let user_db = fs::read_to_string("db.txt").unwrap();
        // serde parses string into vector
        let user_vec = serde_json::from_str(&user_db).unwrap();

        UserController {
            user_vec,
            token: String::new(), // empty string symbolizes that there is no token
            song_controller,
        }
    }

    fn save(&self) {
        // get JSON formatted string from vector
        let serialized = serde_json::to_string(&self.user_vec).unwrap();
        // empty db.txt file
        let mut write = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open("db.txt")
            .expect("Unable to open file");

        // write string to db.txt file
        write
            .write_all(serialized.as_bytes())
            .expect("Unable to write data");
    }

    fn register(&mut self) {
        let mut input_name = user_input("Username: ");

        // check if user already exists with input username
        for user_struct in &self.user_vec {
            // while username already exists, repeat input
            while input_name == user_struct.username {
                println!("Error: User already exists with that username");
                input_name = user_input("Username: ");
            }
        }

        // closure that allows for prompting hidden input
        let pass_fn = || rpassword::prompt_password("Password: ").unwrap();

        let mut input_password = pass_fn();

        // simple password validation
        while input_password.trim().len() < 8 {
            println!("Error: Password must be greater or equal to 8 characters");
            // prompt for password input again
            input_password = pass_fn();
        }

        // salt is a randomly generated 100 character alphanumeric string
        let salt: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(100)
            .map(char::from)
            .collect();

        // argon2 config
        let config = Config::default();

        // hash user input password
        let hashed_password =
            argon2::hash_encoded(input_password.as_bytes(), salt.as_bytes(), &config).unwrap();

        // uuid (random string) that can be used to identify users
        let id: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(50)
            .map(char::from)
            .collect();

        // create a new user
        let new_user = User {
            id,
            username: input_name,
            password: hashed_password,
            favourites: Vec::new(),
        };

        // push to user_vec vector
        self.user_vec.push(new_user);
        // save to file
        self.save();
    }

    // function called whenever there is restricted content (content you need a login token for)
    fn auth_check(&self, warning: bool) -> bool {
        let unregistered_token = self.token.is_empty();
        // if warning is set and token is empty, print error
        if unregistered_token && warning {
            print!("You need to be signed in to perform this action")
        }
        // return true if token is available
        !unregistered_token
    }

    // assume auth_check has been called prior to this function
    fn get_user(&mut self) -> Option<&mut User> {
        // if the saved token is equal to the user id, the user has been identified
        for user in &mut self.user_vec {
            if self.token == user.id {
                return Some(user);
            }
        }
        None
    }

    fn signin(&mut self) {
        let input_name = user_input("Username: ");

        // linear search through user_vec
        for user in &self.user_vec {
            // user identification
            if input_name == user.username {
                // input password closure
                let pass_fn = |label| rpassword::prompt_password(label).unwrap();

                let mut input_password = pass_fn("Password: ");

                // closure to verify if hash matches input password
                let verifier =
                    |input: &str| argon2::verify_encoded(&user.password, input.as_bytes()).unwrap();

                // boolean that defines if password was correct or not
                let mut matches = verifier(&input_password);

                // if password wasn't correct, try again until it is
                while !matches {
                    input_password = pass_fn("Retry Password: ");

                    matches = verifier(&input_password)
                }

                // if password matches, assign active token to the user id
                if matches {
                    println!("Success!");
                    self.token = user.id.clone();

                    return;
                }
            }
        }

        println!("User does not exist! Please try again.");
    }

    fn signout(&mut self) {
        // check if active token is empty or not
        if self.token.is_empty() {
            println!("Not signed in!");
        } else {
            // empty token
            println!("Success!");
            self.token = String::new();
        }
    }

    fn display_favourites(&mut self) {
        // check if authenticated, print error if not
        if self.auth_check(true) {
            // pretty print favourites
            table(&self.get_user().unwrap().favourites);
        }
    }

    fn add_favourite(&mut self) {
        // check if authenticated, print error if not
        if self.auth_check(true) {
            let input_song = user_input("Enter song name: ");

            // use song_controller search method to search for input song
            let search_vec = &self
                .song_controller
                .search("title", Or::String(input_song.to_lowercase()));

            if search_vec.len() == 0 {
                println!("Song not found!");
                return;
            }

            // two songs with the same name could be in the search query. Defaults to first index
            let mut access_idx = 0;

            // prompt input if multiple songs by same name
            if search_vec.len() > 1 {
                table(&search_vec);

                let input_favourite = user_input(&format!(
                    "Choose which one to favourite (1-{}): ",
                    search_vec.len()
                ));

                // human input to access index
                access_idx = input_favourite.parse::<usize>().unwrap() - 1;
            }

            // set favourite to chosen song
            let favourite = search_vec[access_idx].clone();

            // linear search through user favourites and check if chosen favourite already exists
            // in vector
            for i in &self.get_user().unwrap().favourites {
                if i == &favourite {
                    println!("Song already in favourites");
                    return;
                }
            }

            // push favourite to vector
            self.get_user().unwrap().favourites.push(favourite);

            // save to file
            self.save();

            println!("Favourited song!");
        }
    }

    fn remove_favourite(&mut self) {
        // check if authenticated, print error if not
        if self.auth_check(true) {
            // check if user has any favourites
            if &self.get_user().unwrap().favourites.len() == &0 {
                println!("No Favourites");
                return;
            }

            // give option for user to choose what song they want to remove by name
            self.display_favourites();
            let input_song = user_input("Enter Song Name to Remove: ");

            let mut favourite_search = vec![];

            // push to favourite_search vector if favourite name is equal to input
            for favourite in &self.get_user().unwrap().favourites {
                if favourite.get("title") == Or::String(input_song.to_lowercase()) {
                    favourite_search.push(favourite);
                }
            }

            // no favourites found from search query
            if favourite_search.len() == 0 {
                println!("Error: favourite song not found");
                return;
            }

            // index to remove from favourite_search
            let mut remove_idx = 0;

            // prompt input if multiple favourites by same name
            if favourite_search.len() > 1 {
                table(&favourite_search);

                let input_favourite = user_input(&format!(
                    "Choose which one to remove (1-{}): ",
                    favourite_search.len()
                ));

                // human input to index
                remove_idx = input_favourite.parse::<usize>().unwrap() - 1;
            }

            // remove favourite to vector
            self.get_user().unwrap().favourites.remove(remove_idx);

            // save to file
            self.save();

            println!("Successfully removed song from favourites");
        }
    }
}
