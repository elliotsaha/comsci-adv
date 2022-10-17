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
    let mut numbers = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    selection_sort(&mut numbers);
    println!("{:?}", numbers);
}
