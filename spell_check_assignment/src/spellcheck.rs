use crate::utils::{ file_to_vec, capitalize, user_input, linear_search, binary_search };

pub struct SpellChecker {}

pub trait SpellCheckerOperations {
    fn generate_vec(filename: &str) -> Vec<String>;
    fn dictionary(search_method: &str);
    fn story(search_method: &str);
}

impl SpellCheckerOperations for SpellChecker {
    fn generate_vec(filename: &str) -> Vec<String> {
        // transform string into split vector
        let file_type = "txt";
        let dir = "data-files";
        file_to_vec(&format!("{dir}/{filename}.{file_type}"))
    }

    fn dictionary(search_method: &str) {
        println!("{} Search starting...", capitalize(search_method));

        let dictionary = Self::generate_vec("dictionary");

        let target = user_input("Please enter a word").to_lowercase();
        
        let mut found = false;
        
        let callback = |idx: usize| {
            print!("{} is IN dictionary at position {}", target, idx);
            found = true;
        };

        match search_method {
            "linear" => linear_search(&dictionary, &target, callback),
            "binary" => binary_search(&dictionary, &target, callback),
            _ => panic!("Invalid internal method used")
        }

        if !found {
            print!("{} is NOT IN dictionary at position", target);
        }
    }

    fn story(search_method: &str) {
        println!("{} Search starting...", capitalize(search_method));

        let dictionary = Self::generate_vec("dictionary");

        let story = Self::generate_vec("alice");
        
        let mut not_found_count = story.len();

        match search_method {
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

        print!("Number of words not found in dictionary {}", not_found_count);
    }
}
