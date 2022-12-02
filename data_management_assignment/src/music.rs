// used for pretty printing multiple lines of text
use indoc::indoc;
// file system module; OpenOptions used for writing to a file
use std::fs::OpenOptions;
use std::{fs, process};
// JSON module
use serde::{Deserialize, Serialize};
// Tabled trait to signify that a vector of Song structs can be pretty printed
use tabled::Tabled;
// util functions
use crate::utils::{insertion_sort, table, user_input, Or};
// prelude functions
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Tabled, Clone, Debug, PartialEq, Eq)]
pub struct Song {
    pub rank: i16,      // the ranking of the song
    pub title: String,  // song's title
    pub artist: String, // song's artist
    pub album: String,  // the genre of the track
    pub year: i16,      // date song was released
}

pub trait SongTrait {
    // get method is used to dynamically access properties in struct using &str's and outputting to
    // an Or enum
    fn get(&self, key: &str) -> Or;
}

impl SongTrait for Song {
    fn get(&self, key: &str) -> Or {
        match key {
            "rank" => Or::Num(self.rank),
            "title" => Or::String(self.title.to_lowercase()),
            "artist" => Or::String(self.artist.to_lowercase()),
            "album" => Or::String(self.album.to_lowercase()),
            "year" => Or::Num(self.year),
            // return -1 if proper key is not used
            _ => Or::Num(-1),
        }
    }
}

// SongController contains main vector that holds Song structs
#[derive(Serialize, Deserialize, Clone)]
pub struct SongController {
    song_vec: Vec<Song>,
}

pub trait SongOperations {
    fn new() -> Self;
    // post to data file
    fn save(&self);
    // displays all songs
    fn display_all(&self);
    // utility class function for searching through songs using linear search. Used in
    // user_controller class as well
    fn search(&self, key: &str, match_value: Or) -> Vec<&Song>;
    // prompts menu for user to select key to search for songs
    fn search_menu(&self);
    // sorts songs by a certain key
    fn sort(&mut self);
}

impl SongOperations for SongController {
    fn new() -> Self {
        // reads songs.txt contents as string
        let top_songs = fs::read_to_string("songs.txt").unwrap();
        // serde parses string into vector
        let song_vec = serde_json::from_str(&top_songs).unwrap();
        // initialize SongController with vector of songs from data file
        SongController { song_vec }
    }

    // writes vector as JSON formatted string to songs.txt
    fn save(&self) {
        // get JSON formatted string from vector
        let serialized = serde_json::to_string(&self.song_vec).unwrap();
        // empty out songs.txt file
        let mut write = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open("songs.txt")
            .expect("Unable to open file");

        // write string to songs.txt file
        write
            .write_all(serialized.as_bytes())
            .expect("Unable to write data");
    }

    // pretty prints all songs
    fn display_all(&self) {
        table(&self.song_vec);
    }

    fn search(&self, key: &str, match_value: Or) -> Vec<&Song> {
        let mut results: Vec<&Song> = vec![];

        // push to results if song value associated with input key is equal to match_value
        for song in &self.song_vec {
            if song.get(key) == match_value {
                results.push(song);
            }
        }

        results
    }

    fn search_menu(&self) {
        // gives menu for user to select what key they want to search by
        let category_input = user_input(indoc! {"
            SEARCH BY:
            1. Rank
            2. Title 
            3. Artist
            4. Album 
            5. Year
            6. Exit
        "});

        // exit before next query_input is taken. Not a part of match statement because user_input
        // will be asked before returning
        if category_input == "6" {
            return;
        }

        let query_input = user_input("Enter Search Query: ");

        // closure that does simple validation if input is too large to be parsed (avoids program
        // crashing)
        let num_ord = |key| {
            // simple way to avoid integer overflow
            // len > 4 because thats the max amount of digits present in a standard year, anything
            // higher may be an attempt to crash the program
            if query_input.len() > 4 {
                println!("Error: Invalid input!");
                return;
            } else {
                // Convert input into a number, linear search for input in song data and pretty print the search vector
                // unwrap_or_default takes poorly formed strings and converts to 0 which leads to no results found
                table(&self.search(key, Or::Num(query_input.parse().unwrap_or_default())))
            }
        };

        // Convert input into a lowercase string, linear search for input in song data and pretty print the search vector
        let str_ord = |key| table(&self.search(key, Or::String(query_input.to_lowercase())));

        match category_input.as_str() {
            "1" => num_ord("rank"),
            "2" => str_ord("title"),
            "3" => str_ord("artist"),
            "4" => str_ord("album"),
            "5" => num_ord("year"),
            _ => println!("Invalid Operation"),
        }
    }

    // sort function mutates original vector but DOES NOT save to file
    fn sort(&mut self) {
        let sort_input = user_input(indoc! {"
            SORT BY:
            1. Rank
            2. Title 
            3. Artist
            4. Album 
            5. Year
            6. Exit
        "});

        // closure that does insertion sort with a certain key
        let mut sorted = |key| insertion_sort(&mut self.song_vec, key);

        match sort_input.as_str() {
            "1" => sorted("rank"),
            "2" => sorted("title"),
            "3" => sorted("artist"),
            "4" => sorted("album"),
            "5" => sorted("year"),
            "6" => process::exit(1),
            _ => println!("Invalid Operation"),
        }

        // display vector after sorting
        self.display_all();
    }
}
