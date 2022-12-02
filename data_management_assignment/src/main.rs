use music::{SongController, SongOperations};
use user::{UserController, UserOperations};
use utils::main_menu;

pub mod music;
pub mod user;
pub mod utils;

fn main() {
    let mut song_controller = SongController::new();
    // song_controller class clone needs to be passed to user_controller for accessing song vector
    // and initialization of classes should not be done with references
    let mut user_controller = UserController::new(song_controller.clone());

    loop {
        main_menu(&mut song_controller, &mut user_controller);
    }
}
