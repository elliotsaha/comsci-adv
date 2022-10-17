use crate::menu::display_menu;

pub mod menu;
pub mod utils;
pub mod spellcheck;

fn main() {
    loop {
        display_menu();
    }
}
