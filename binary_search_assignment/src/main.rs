fn binary_search<T>(list: &[T], target: T) -> isize
// each generic element must have PartialOrd type for checking equivalences
// and copy for dereferencing
where T: std::cmp::PartialOrd + Copy {
    let mut start = 0;
    let mut end = list.len() - 1;
    let middle = ((end / 2) as f32).floor() as usize;

    while start < end {
        if list[middle] == target {
            return middle as isize;
        }
        if list[middle] > target {
            end = middle;
        } else {
            start = middle;
        }
    }
    // return -1 if not found
    -1
}

fn main() {
    let list  = vec![1,9,20,40,100,110,300,320];
    println!("{}", binary_search(&list, 300));
}
