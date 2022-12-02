use std::io;

struct Color {
    name: String,
    hex: String,
    family: String,
    r: u8,
    g: u8,
    b: u8,
    brightness: u8,
}
fn main() {
    let color_data: [Color; 3] = [
        Color {
            name: String::from("Lavender"),
            hex: String::from("E6E6FA"),
            family: String::from("Purple"),
            r: 230,
            g: 230,
            b: 250,
            brightness: 231,
        },
        Color {
            name: String::from("Crimson"),
            hex: String::from("FF0000"),
            family: String::from("Red"),
            r: 230,
            g: 230,
            b: 250,
            brightness: 231,
        },
        Color {
            name: String::from("Orchid"),
            hex: String::from("800080"),
            family: String::from("Purple"),
            r: 230,
            g: 230,
            b: 250,
            brightness: 231,
        },
    ];

    for color in &color_data {
        println!("Name: {}, Family: {}", color.name, color.family);
    }

    for color in &color_data {
        if color.brightness >= 200 {
            println!("Name: {}", color.name);
        }
    }

    let mut count = 0;
    for color in &color_data {
        if color.family == "Pink" || color.family == "Red" {
            count += 1;
        }
    }
    println!("{count}");

    println!("Enter family name:");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let trimmed_input = input.trim();

    let mut family_count = 0;
    for color in &color_data {
        println!("All colors that belong to the {trimmed_input} family:");

        if trimmed_input.to_lowercase() == color.family.to_lowercase() {
            println!("Name: {}, Family: {}", color.name, color.family);

            family_count += 1;
        }
    }
    println!("count: {family_count}");

    println!("Single letter input:");
    let mut letter_input = String::new();

    io::stdin()
        .read_line(&mut letter_input)
        .expect("Failed to read line");

    let trimmed_input = letter_input.trim();

    let mut count_nth = 0;
    for color in &color_data {
        println!("All colors that start with input character");

        if &trimmed_input.to_lowercase()
            == &color
                .name
                .to_lowercase()
                .chars()
                .nth(0)
                .unwrap()
                .to_string()
        {
            println!("{}", color.name);
            count_nth += 1;
        }
    }
    println!("{count_nth}");
}
