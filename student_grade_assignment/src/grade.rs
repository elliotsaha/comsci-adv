// library to generate random numbers
use rand::Rng;

pub struct GradeBook {
    grades: Vec<u8>
}

// GradeBook methods
pub trait GradeOperations {
    fn gen_grades() -> Vec<u8>; // static function used in randomize() and new()
    fn new() -> Self;
    fn show_all(&self);
    fn honours(&self);
    fn max(&self) -> &u8;
    fn min(&self) -> &u8;
    fn avg(&self) -> u8;
    fn stats(&self);
    fn randomize(&mut self);
}

impl GradeOperations for GradeBook {
    // used as initializer for class (runs with constructor setting grades to gen_grades())
    fn new() -> GradeBook {
        GradeBook { grades: Self::gen_grades() }
    }

    // generates 35 random values from range 0-100
    fn gen_grades() -> Vec<u8> {
        let mut rng = rand::thread_rng();

        let mut random_grades: Vec<u8> = Vec::with_capacity(35);

        for _ in 0..35 {
            random_grades.push(rng.gen_range(0..101)); // gen_range fn returns [0, 101) -> will not include 101
        }
        
        // returns vector with a size of 35 with integers ranging from 0 to 100
        random_grades
    }

    // shows all values in grades array
    fn show_all(&self) {
        println!("ALL GRADES");
        for grade in &self.grades {
            println!("{}%", grade);
        }
    }

    // shows all values that are equal to or above 80 in grades array
    fn honours(&self) {
        println!("HONOURS");
        let mut honour_count: u8 = 0;
        for grade in &self.grades {
            if grade >= &80 {
                println!("{}%", grade);
                honour_count += 1;
            }
        }
        println!("Number of Honours: {}", honour_count);
    }

    // gets maximum value from grades array
    fn max(&self) -> &u8 {
        self.grades.iter().max().unwrap()
    }

    // gets minimum value from grades array
    fn min(&self) -> &u8 {
        self.grades.iter().min().unwrap()
    }

    // gets mean value from grades array
    fn avg(&self) -> u8 {
        // sum (which can hold a max value of 350) must match the vector iterator type so each individual grade in array is
        // dereferenced and casted as u16
        let sum: u16 = self.grades.iter().map(|grade| (*grade) as u16).sum();
        let mean: f32 = sum as f32 / self.grades.len() as f32;
        // returns u8 value because rounded mean value will be from 0 - 100
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
