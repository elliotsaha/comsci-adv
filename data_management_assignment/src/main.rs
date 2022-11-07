use music::{SongController, SongOperations};
use user::{UserController, UserOperations};

pub mod music;
pub mod user;
pub mod utils;

fn main() {
    // let mut user = UserController::new();
    // user.register();
    let music = SongController::new();
    music.display_all();
}
