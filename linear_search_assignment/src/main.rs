fn linear_search<T>(list: &[T], target: T) -> isize 
// each generic element must have PartialOrd type for checking equivalences
// and copy for dereferencing
where T: std::cmp::PartialOrd + Copy {
    // loop through vector iteratively
    for (idx, i) in list.iter().map(|i| (*i) as T).enumerate() {
        // check if target element is equal to iteration
        if i == target {
            // return index of element if match
            return idx as isize;
        }
    }
    // return -1 if not found
    -1
}

fn main() {
    let list = vec![1, 4, 7, 9, 20, 110, 200, 300, 900, 1200];
    linear_search(&list, 900);
}
