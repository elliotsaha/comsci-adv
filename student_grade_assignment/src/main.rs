use crate::grade::{ GradeBook, GradeOperations };
use crate::menu::display_menu;

pub mod grade;
pub mod menu;

fn main() {
    // initialize GradeBook
    let mut grade_book = GradeBook::new();

    // infinite loop for display_menu (option 5 in display_menu will close program entirely)
    loop {
        display_menu(&mut grade_book);
    }
}
