// file system module
use std::fs;
// JSON module
use serde::Deserialize;
use tabled::{Style, Table, Tabled};

#[derive(Deserialize, Tabled)]
pub struct Song {
    rank: u16,      // the ranking of the song
    title: String,  // song's title
    artist: String, // song's artist
    genre: String,  // the genre of the track
    year: u16,      // date song was released
    bpm: u16,       // beats per minute - the tempo of the song
}

#[derive(Deserialize)]
pub struct SongController {
    song_vec: Vec<Song>,
}

pub trait SongOperations {
    fn new() -> Self;
    fn display_all(&self); // display all of the data
    fn pretty_table<T: Tabled>(vec: &Vec<T>);
    //    fn filter_menu(&self); // displays menu for user to filter out data
    //    fn sort(&self); // sort data
}

impl SongOperations for SongController {
    fn new() -> Self {
        // reads top600songs.txt contents as string
        let top_songs = fs::read_to_string("top600songs.txt").unwrap();
        // serde parses string into vector
        let song_vec = serde_json::from_str(&top_songs).unwrap();

        SongController { song_vec }
    }
    // make pretty table from Vec<Song>
    fn pretty_table<T: Tabled>(vec: &Vec<T>) {
        let mut table = Table::new(vec);
        table.with(Style::empty());
        println!("{table}");
    }

    fn display_all(&self) {
        Self::pretty_table(&self.song_vec);
    }
}
