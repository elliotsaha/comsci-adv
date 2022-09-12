use crate::list::{ List, ListOperations };

pub mod list;

fn main() {
    let mut new_list = List { vector: vec![1,5,8,2,1,2,3,5,9,3,0,1,8] };
    println!("GET METHOD: {:?}", new_list.get());
    println!("CONTAINS(9) METHOD: {}", new_list.contains(9));
    println!("INDEX_OF(8) METHOD: {}", new_list.index_of(8));
    println!("REVERSE METHOD: {:?}", new_list.reverse());
    // SWAP METHOD
    new_list.swap(0, 3);
    println!("GET METHOD: (AFTER SWAP): {:?}", new_list.get());
    println!("INDEX_OF_MIN METHOD: {}", new_list.index_of_min());
}

