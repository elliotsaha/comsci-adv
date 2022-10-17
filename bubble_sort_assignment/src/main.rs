fn bubble_sort<T: Ord>(arr: &mut [T]) {
    // iterate through entire array
    for i in 0..arr.len() - 1 {
        let mut swapped = false;

        // subtracting by i to prevent unnecessary checks
        // on values that can be guaranteed to be swapped already
        // (largest values are naturally carried to end of array)
        for j in 0..arr.len() - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }

        // if no swaps have taken place, terminate function
        if !swapped { return; }
    }
}

fn main() {
    let mut words = ["d", "c", "b", "a"];
    let mut nums = [9, 2, 1, 0, 7, 3];

    bubble_sort(&mut words);
    bubble_sort(&mut nums);

    assert_eq!(words, ["a", "b", "c", "d"]);
    assert_eq!(nums, [0, 1, 2, 3, 7, 9]);
}
