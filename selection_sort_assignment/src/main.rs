fn selection_sort<T: Ord>(arr: &mut [T]) {

   for i in 0..arr.len() - 1 {
       // search for minimum
       let mut min = i;
       for j in (i + 1)..arr.len() {
           if arr[j] < arr[min] {
               // set current to minimum
               min = j;
           }
       }
       arr.swap(min, i)
   }
}

fn main() {
    let mut numbers = [9, 2, 5, 1, -4, 6, -7];
    selection_sort(&mut numbers);
    assert_eq!(numbers, [-7, -4, 1, 2, 5, 6, 9]);
}
