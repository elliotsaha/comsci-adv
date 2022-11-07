use crate::contact::{ContactOperations, Contacts};
use crate::menu::display_menu;

pub mod contact;
pub mod menu;
pub mod utils;

fn main() {
    let mut contacts = Contacts::new();

    loop {
        display_menu(&mut contacts);
    }
}
