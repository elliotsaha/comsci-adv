use std::fs;

fn file_to_vec(filename: &str) -> Vec<String> {
    let file_to_string = fs::read_to_string(filename).unwrap();
    file_to_string.split_whitespace().map(|x| x.to_owned()).collect()

}

fn main() {
    println!("{:?}", file_to_vec("data-files/dictionary.txt"));
}
