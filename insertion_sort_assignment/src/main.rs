fn insertion_sort<T: Ord + Copy>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let curr_val = arr[i];
        let mut curr_pos = i;

        while curr_pos > 0 && arr[curr_pos - 1] > curr_val {
            arr[curr_pos] = arr[curr_pos - 1];
            curr_pos -= 1;
        }

        arr[curr_pos] = curr_val;
    }
}

fn main() {
    let mut nums = [10, 70, 30, 100, 40, 45, 90, 80, 85];
    let mut words = ["dog","at", "good", "eye", "cat", "ball", "fish"];

    insertion_sort(&mut nums);
    insertion_sort(&mut words);

    assert_eq!(nums, [10, 30, 40, 45, 70, 80, 85, 90, 100]);
    assert_eq!(words, ["at", "ball", "cat", "dog", "eye", "fish", "good"]);
}
