// utility function imports
use crate::utils::{ file_to_vec, capitalize, user_input, linear_search, binary_search };
// get time duration of function
use std::time::Instant;

pub struct SpellChecker {}

pub trait SpellCheckerOperations {
    // spell check a word
    fn dictionary(search_method: &str);
    // spell check story
    fn story(search_method: &str);
}

impl SpellCheckerOperations for SpellChecker {
    fn dictionary(search_method: &str) {
        println!("{} Search starting...", capitalize(search_method));
        
        let dictionary = file_to_vec("dictionary");
        
        let target = user_input("Please enter a word").to_lowercase();
        
        let mut found = false;
        
        // start timer
        let now = Instant::now();

        let callback = |idx: usize| {
            // end timer if target is found
            print!("{target} is IN dictionary at position {idx} ({} seconds)", now.elapsed().as_secs_f64());
            found = true;
        };

        match search_method {
            "linear" => linear_search(&dictionary, &target, callback),
            "binary" => binary_search(&dictionary, &target, callback),
            _ => panic!("Invalid internal method used")
        }

        if !found {
            // alternative end timer if target is not found
            print!("{target} is NOT IN dictionary at position ({} seconds)", now.elapsed().as_secs_f64());
        }
    }

    fn story(search_method: &str) {
        println!("{} Search starting...", capitalize(search_method));

        let dictionary = file_to_vec("dictionary");

        let story = file_to_vec("alice");
        
        let mut not_found_count = story.len();

        // start timer
        let now = Instant::now();

        match search_method {
            // decrement not_found_count when word is found in dictionary
            // NOTE: cannot create a closure out of scope because closure needs to be copied
            // on every iteration
            "linear" => {
                for word in &story {
                    linear_search(&dictionary, &word.to_lowercase(), |_| not_found_count -= 1);
                }
            },
            "binary" => {
                for word in &story {
                    binary_search(&dictionary, &word.to_lowercase(), |_| not_found_count -= 1);
                }
            },
            _ => panic!("Invalid internal method used")
        }

        // end timer
        print!("Number of words not found in dictionary: {not_found_count} ({} seconds)", now.elapsed().as_secs_f64());
    }
}
