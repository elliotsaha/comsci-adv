// utility functions
use crate::utils::{
    error, exit_header, hidden_user_input, req_exit, success, table, user_input, Or,
};
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
        // inform user on how to exit input
        exit_header();

        // initialize username and password variables
        let username;
        let password;

        // uuid (random string) that can be used to identify users
        let id: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(50)
            .map(char::from)
            .collect();

        loop {
            let input_name = user_input("Username: ");

            let mut pass_check = true;

            if req_exit(&input_name) {
                return;
            }

            for user in &self.user_vec {
                // if user already exists
                if input_name == user.username {
                    error("User already exists with that username");
                    // retry username prompt
                    pass_check = false;
                }
            }

            if pass_check {
                username = input_name;
                break;
            }
        }

        loop {
            // prompt for hidden input
            let input_password = hidden_user_input("Password: ");

            if req_exit(&input_password) {
                return;
            }

            // simple password validation
            if input_password.trim().len() < 8 {
                error("Password must be greater or equal to 8 characters");
            } else {
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
                    argon2::hash_encoded(input_password.as_bytes(), salt.as_bytes(), &config)
                        .unwrap();

                password = hashed_password;

                break;
            }
        }

        // create a new user
        let new_user = User {
            id,
            username,
            password,
            favourites: Vec::new(),
        };

        // push to user_vec vector
        self.user_vec.push(new_user);
        // save to file
        self.save();

        success();
    }

    // function called whenever there is restricted content (content you need a login token for)
    fn auth_check(&self, warning: bool) -> bool {
        let unregistered_token = self.token.is_empty();
        // if warning is set and token is empty, print error
        if unregistered_token && warning {
            error("You need to be signed in to perform this action");
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
        exit_header();

        // index of user struct found in self.user_vec (initially set to 0)
        let mut user_idx = 0;

        loop {
            let input_name = user_input("Username: ");

            let mut pass_check = false;

            if req_exit(&input_name) {
                return;
            }

            for (idx, user) in self.user_vec.iter().enumerate() {
                // user identified
                if input_name == user.username {
                    // set proper idx
                    user_idx = idx;
                    pass_check = true;
                }
            }

            if pass_check {
                break;
            } else {
                error("User does not exist! Please try again.");
            }
        }

        // identified user object
        let user = &self.user_vec[user_idx];

        loop {
            let input_password = hidden_user_input("Password (Hidden Input): ");

            if req_exit(&input_password) {
                return;
            }

            // if input password hashed matches the stored hash
            let password_match =
                argon2::verify_encoded(&user.password, input_password.as_bytes()).unwrap();

            if password_match {
                break;
            } else {
                error("Invalid Password. Please try again.");
            }
        }

        // if password matches, assign active token to the user id
        self.token = user.id.clone();

        success();
    }

    fn signout(&mut self) {
        // check if active token is empty or not
        if self.token.is_empty() {
            error("Not signed in!");
        } else {
            // empty token
            self.token = String::new();
            success();
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
            exit_header();

            // vector containing songs that user has possibly searched for
            // (multiple songs could have same name)
            let search_vec;

            loop {
                let input_song = user_input("Enter song name: ");

                let mut pass_check = true;
                if req_exit(&input_song) {
                    return;
                }

                // use song_controller search method to search for input song
                let song_search = self
                    .song_controller
                    .search("title", Or::String(input_song.to_lowercase()));

                if song_search.len() == 0 {
                    pass_check = false;
                }

                if pass_check {
                    search_vec = song_search;
                    break;
                } else {
                    error("Song not found! Please try again.");
                }
            }

            // two songs with the same name could be in the search query. Defaults to first index
            let mut access_idx = 0;

            // prompt input if multiple songs by same name
            if search_vec.len() > 1 {
                table(&search_vec);

                loop {
                    let input_favourite = user_input(&format!(
                        "Choose which one to favourite (1-{}): ",
                        search_vec.len()
                    ));

                    if req_exit(&input_favourite) {
                        return;
                    }

                    // check if input can be parsed
                    if input_favourite.parse::<usize>().is_ok() {
                        // human input to access index
                        access_idx = input_favourite.parse::<usize>().unwrap() - 1;
                        break;
                    } else {
                        error("Invalid input. Please try again.");
                    }
                }
            }

            // set favourite to chosen song
            let favourite = search_vec[access_idx].clone();

            // linear search through user favourites and check if chosen favourite already exists
            // in vector
            for i in &self.get_user().unwrap().favourites {
                if i == &favourite {
                    // assuming user thought that favourited song wasn't in favourites already and
                    // go back to main menu
                    error("Song already in favourites");
                    return;
                }
            }

            // push favourite to favourites vector
            self.get_user().unwrap().favourites.push(favourite);

            // save to file
            self.save();

            success();
        }
    }

    fn remove_favourite(&mut self) {
        // check if authenticated, print error if not
        if self.auth_check(true) {
            // check if user has any favourites
            if self.get_user().unwrap().favourites.len() == 0 {
                error("No Favourites");
                return;
            }

            // give option for user to choose what song they want to remove by name
            self.display_favourites();

            exit_header();

            // vector containing possible songs that user is trying to remove
            // (multiple songs could have same name)
            let mut favourites_vec = vec![];

            loop {
                let input_song = user_input("Enter Song Name to Remove: ");

                if req_exit(&input_song) {
                    return;
                }

                // push to favourites_vec vector if favourite name is equal to input
                for favourite in self.get_user().unwrap().favourites.clone() {
                    if favourite.get("title") == Or::String(input_song.to_lowercase()) {
                        favourites_vec.push(favourite);
                    }
                }

                if favourites_vec.len() != 0 {
                    break;
                } else {
                    error("Favourite song not found");
                }
            }

            // index to remove from favourites_vec
            let mut remove_idx = 0;

            // prompt input if multiple favourites by same name
            if favourites_vec.len() > 1 {
                table(&favourites_vec);

                loop {
                    let input_favourite = user_input(&format!(
                        "Choose which one to remove (1-{}): ",
                        favourites_vec.len()
                    ));

                    if req_exit(&input_favourite) {
                        return;
                    }

                    // check if input can be parsed
                    if input_favourite.parse::<usize>().is_ok() {
                        // human input to access index
                        remove_idx = input_favourite.parse::<usize>().unwrap() - 1;
                        break;
                    } else {
                        error("Invalid input. Please try again.");
                    }
                }
            }

            // remove favourite to vector
            self.get_user().unwrap().favourites.remove(remove_idx);

            // save to file
            self.save();

            success();
        }
    }
}
