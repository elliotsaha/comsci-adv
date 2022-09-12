// input output module used for taking input from user
use std::io;
// process module used for exiting program
use std::process;
// library to generate random numbers
use rand::Rng;

struct GradeBook {
    grades: Vec<u16>
}

// GradeBook methods
trait GradeOperations {
    fn gen_grades() -> Vec<u16>; // static function used in randomize() and new()
    fn new() -> Self;
    fn show_all(&self);
    fn honours(&self);
    fn max(&self) -> &u16;
    fn min(&self) -> &u16;
    fn avg(&self) -> u8;
    fn stats(&self);
    fn randomize(&mut self);
}

impl GradeOperations for GradeBook {
    // generates 35 random values from range 0-100
    fn gen_grades() -> Vec<u16> {
        let mut rng = rand::thread_rng();

        let mut rand_grades: Vec<u16> = Vec::with_capacity(35);

        for _ in 0..35 {
            rand_grades.push(rng.gen_range(0..101)); // gen_range fn returns [0, 101) -> will not include 101
        }

        rand_grades
    }

    // used as initializer for class (runs with constructor setting grades to gen_grades())
    fn new() -> GradeBook {
        GradeBook { grades: Self::gen_grades() }
    }

    // shows all values in grades array
    fn show_all(&self) {
        println!("ALL GRADES");
        for i in &self.grades {
            println!("{}%", i)
        }
    }

    // shows all values that are equal to or above 80 in grades array
    fn honours(&self) {
        println!("HONOURS");
        let mut count = 0;
        for i in &self.grades {
            if i >= &80 {
                println!("{}%", i);
                count += 1;
            }
        }
        println!("Number of Honours: {}", count);
    }

    // gets maximum value from grades array
    fn max(&self) -> &u16 {
        self.grades.iter().max().unwrap()
    }

    // gets minimum value from grades array
    fn min(&self) -> &u16 {
        self.grades.iter().min().unwrap()
    }

    // gets mean value from grades array
    fn avg(&self) -> u8 {
        let sum: u16 = self.grades.iter().sum();
        let mean: f32 = sum as f32 / self.grades.len() as f32;
        mean.round() as u8
    }

    // prints general stats about grades array
    fn stats(&self) {
        println!("STATS");
        println!("Highest Grade: {}%", self.max());
        println!("Lowest Grade: {}%", self.min());
        println!("Average Grade: {}%", self.avg());
    }

    // calls gen_grades() to randomize grades array
    fn randomize(&mut self) {
        let new_grades = Self::gen_grades();
        self.grades = new_grades;
        println!("GRADES HAVE BEEN RANDOMIZED");
    }
}

fn display_menu(grade_book: &mut GradeBook) {
    println!(r#"
MAIN MENU
    1. Display All Grades
    2. Display Honours
    3. Stats
    4. Randomize Grades
    5. Exit
    "#);

    let mut input = String::new();

    // make "input" variable = user input from terminal
    io::stdin().read_line(&mut input).expect("Failed to read line");

    println!("\n");
    // remove trailing whitespace from input
    // (input from keyboard will always contain trailing newline character)
    match input.trim() {
        "1" => grade_book.show_all(),
        "2" => grade_book.honours(),
        "3" => grade_book.stats(),
        "4" => grade_book.randomize(),
        "5" => process::exit(1),
        _ => println!("Invalid choice"),
    }
}

fn main() {
    // initialize GradeBook
    let mut grade_book = GradeBook::new();

    // infinite loop for display_menu (option 5 in display_menu will close program entirely)
    loop {
        display_menu(&mut grade_book);
    }
}
