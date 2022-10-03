use std::cmp::Ordering;

fn binary_search<T>(list: &[T], target: T) -> isize
// each generic element must have Ord trait for match comparison 
// and copy for dereferencing
where T: std::cmp::Ord + Copy {
    let mut start = 0;
    let mut end = list.len() - 1;
    
    // search all possible candidates for target in vector
    while start <= end {
        // get middle index of vector
        let middle = (((start + end) / 2) as f32).floor() as usize;
        
        match target.cmp(&list[middle]) {
                Ordering::Equal => {
                    return middle as isize;
                }
                Ordering::Less => {
                    // search left half of vector
                    end = middle - 1;
                }
                Ordering::Greater => {
                    // search right half of vector
                    start = middle + 1;
                }
        }
    }
    // return -1 if not found
    -1
}

fn main() {
    let list  = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"];
    println!("{}", binary_search(&list, "j"));
}
