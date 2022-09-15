fn binary_search<T>(list: &[T], target: T) -> isize
// each generic element must have PartialOrd type for checking equivalences
// and copy for dereferencing
where T: std::cmp::PartialOrd + Copy {
    let mut start = 0;
    let mut end = list.len() - 1;
    
    // search all possible candidates for target in vector
    while start <= end {
        // get middle index of vector
        let middle = (((start + end) / 2) as f32).floor() as usize;
        if target == list[middle] {
            return middle as isize;
        } else if target < list[middle] {
            // search left half of vector
            end = middle - 1;
        } else {
            // search right half of vector
            start = middle + 1;
        }
    }
    // return -1 if not found
    -1
}

fn main() {
    let list  = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"];
    println!("{}", binary_search(&list, "m"));
}
