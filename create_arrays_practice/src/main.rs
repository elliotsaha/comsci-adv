use rand::Rng;
use std::io;

fn main() {
    // for any rng based operations
    let mut rng = rand::thread_rng();

    let months_of_year_with_j = ["January", "June", "July"];
    println!("{:?}", months_of_year_with_j);

    let mut joy_list = vec![];

    for _ in 0..700 {
        joy_list.push("joy");
    }

    println!("{:?}", joy_list);

    let mut five_k_rand_dec = vec![];

    for _ in 0..5000 {
        five_k_rand_dec.push(rng.gen_range(0.0..100.0));
    }

    println!("{:?}", five_k_rand_dec);

    let mut three_h_rand_dec = vec![];

    for _ in 0..300 {
        three_h_rand_dec.push(rng.gen_range(0.0..40.0));
    }

    println!("{:?}", three_h_rand_dec);

    let mut multiple_list = vec![];

    let mut multiple = 20;

    while multiple <= 800 {
        multiple_list.push(multiple);
        multiple += 4;
    }

    println!("{:?}", multiple_list);

    let mut even_nums = vec![];

    let mut even = 100;

    while even >= 10 {
        even_nums.push(even);
        even -= 2;
    }

    println!("{:?}", even_nums);

    let colors_list: Vec<&str> = "red,orange,yellow,green,blue,indigo,violet"
        .split(",")
        .collect();

    println!("{:?}", colors_list);

    let cities_list: Vec<&str> = "Edmonton;Calgary;Vancouver;Saskatoon;Winnipeg"
        .split(";")
        .collect();

    println!("{:?}", cities_list);

    let mut user_input_vec: Vec<String> = vec![];

    loop {
        println!("ADD TO ARRAY:");
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed input");

        user_input = String::from(user_input.trim());

        if user_input == "done" {
            break;
        }

        user_input_vec.push(user_input);

        println!("{:?}", user_input_vec);
    }
}
