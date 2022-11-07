fn main() {
    let results = ["Yes", "Yes", "No", "Maybe", "Maybe", "Yes", "No", "Maybe"];

    let mut yes_count: u8 = 0;
    let mut no_count: u8 = 0;
    let mut maybe_count: u8 = 0;

    for response in results {
        match response {
            "Yes" => yes_count += 1,
            "No" => no_count += 1,
            _ => maybe_count += 1,
        }
    }

    println!("YES: {yes_count}, NO: {no_count}, MAYBE: {maybe_count}");

    let ages = [18, 20, 7, 24, 30, 9, 12, 10];

    let mut adult_count: u8 = 0;
    let mut minor_count: u8 = 0;

    for age in ages {
        if age >= 18 {
            adult_count += 1;
        } else {
            minor_count += 1;
        }
    }

    println!("ADULT: {adult_count}, MINOR: {minor_count}");

    let mut prices = vec![
        1243.0, 72342.0, 123123.0, 12512.0, 412.0, 4.0, 123359.0, 1232.0, 122.0, 20.0, 40.0, 30.0,
        8.0,
    ];

    let mut under_20_count: u8 = 0;
    let mut range_20to49_count: u8 = 0;
    let mut equal_or_above50_count: u8 = 0;

    for price in &mut prices {
        if *price >= 50.0 {
            equal_or_above50_count += 1;
        } else if *price >= 20.0 {
            range_20to49_count += 1;
        } else {
            under_20_count += 1;
        }

        *price += 2.0;
        *price *= 1.05;
        *price *= 0.80;
        *price = (format!("{:.2}", *price)).parse().unwrap();
    }

    println!("UNDER_20: {under_20_count}, RANGE_20to49: {range_20to49_count}, EQUAL_OR_ABOVE50: {equal_or_above50_count}");
    println!("{:?}", prices);
}
