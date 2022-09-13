use crate::nickname::{ NicknameGenerator, NicknameOperations };
use crate::menu::display_menu;

pub mod menu;
pub mod utils;
pub mod nickname;

fn main() {
    let mut nickname_generator = NicknameGenerator::new();
    
    nickname_generator.prompt_name();

    loop {
        display_menu(&mut nickname_generator);
    }
}
