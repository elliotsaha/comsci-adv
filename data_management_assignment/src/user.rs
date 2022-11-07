// get user input helper
use crate::utils::user_input;
// file system module
use std::fs::{self, OpenOptions};
// JSON module
use serde::{Deserialize, Serialize};
// prelude functions
use std::io::prelude::*;
// argon2 is a password hashing algorithm
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserController {
    user_vec: Vec<User>,
    active: bool, // if the user is logged in
}

pub trait UserOperations {
    fn new() -> Self;
    fn save(&self);
    fn register(&mut self); // new user
}

impl UserOperations for UserController {
    fn new() -> Self {
        // reads db.txt contents as string
        let user_db = fs::read_to_string("db.txt").unwrap();
        // serde parses string into vector
        let user_vec = serde_json::from_str(&user_db).unwrap();

        UserController {
            user_vec,
            active: false,
        }
    }

    fn save(&self) {
        // get JSON formatted string from vector
        let serialized = serde_json::to_string(&self.user_vec).unwrap();
        // empty contactList file
        let mut write = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open("db.txt")
            .expect("Unable to open file");

        // write vector as string to contactList file
        write
            .write_all(serialized.as_bytes())
            .expect("Unable to write data");
    }

    fn register(&mut self) {
        println!("CREATE NEW USER");
        let mut input_name = user_input("USERNAME: ");

        // check if user already exists
        for user_struct in &self.user_vec {
            while input_name == user_struct.username {
                println!("ERR: User already exists with that username");
                input_name = user_input("USERNAME: ");
            }
        }

        let pass_fn = || rpassword::prompt_password("PASSWORD: ").unwrap();

        let mut input_password = pass_fn();

        // simple password validation
        while input_password.trim().len() < 8 {
            println!("ERR: Password must be greater or equal to 8 characters");
            // prompt for password input again
            input_password = pass_fn();
        }

        // generate hash
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();

        let hashed_password = argon2
            .hash_password(input_password.as_bytes(), &salt)
            .expect("Error hashing password")
            .to_string();

        // create new user with hashed password
        let new_user = User {
            username: input_name,
            password: hashed_password,
        };

        self.user_vec.push(new_user);
        self.save();
    }
}
