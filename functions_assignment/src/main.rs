use std::convert::TryInto;

struct List {
   vector: Vec<isize> // guarenteed to have enough size for any architecture
}

trait ListOperations {
    fn get(&self) -> &Vec<isize>;
    fn contains(&self, item: isize) -> bool;
    fn index_of(&self, item: isize) -> isize;
    fn reverse(&self) -> Vec<isize>;
    fn swap(&mut self, idx1: usize, idx2: usize);
    fn index_of_min(&self) -> usize;
}

impl ListOperations for List {
    fn get(&self) -> &Vec<isize> {
        &self.vector
    }

    fn contains(&self, item: isize) -> bool {
        let mut contains = false;
        for i in &self.vector {
            if i == &item {
                contains = true;
            }
        }
        contains
    }

    fn index_of(&self, item: isize) -> isize {
        for (idx, &i) in self.vector.iter().enumerate() {
            if i == item {
                return idx.try_into().unwrap();
            }
        }
        -1
    }

    fn reverse(&self) -> Vec<isize> {
        let mut idx: isize = (self.vector.len() - 1) as isize;
        let mut reverse_vec = Vec::new(); // reversed vector should have same capacity as initial vector
        while idx >= 0 {
            reverse_vec.push(self.vector[idx as usize]);
            idx -= 1;
        }
        reverse_vec
    }

    fn swap(&mut self, idx1: usize, idx2: usize) {
        // create temporary copies of each value being swapped
        let temp_idx1 = self.vector[idx1];
        let temp_idx2 = self.vector[idx2];
        self.vector[idx1] = temp_idx2;
        self.vector[idx2] = temp_idx1;
    }

    fn index_of_min(&self) -> usize {
        let mut min_idx = 0;
        for (idx, &i) in self.vector.iter().enumerate() {
            if i < self.vector[min_idx] {
                min_idx = idx;
            }
        }
        min_idx
    }
}

fn main() {
    let mut new_list = List { vector: vec![1,5,8,2,1,2,3,5,9,3,0,1,8] };
    println!("GET METHOD: {:?}", new_list.get());
    println!("CONTAINS(9) METHOD: {}", new_list.contains(9));
    println!("INDEX_OF(8) METHOD: {}", new_list.contains(8));
    println!("REVERSE METHOD: {:?}", new_list.reverse());
    // SWAP METHOD
    new_list.swap(0, 3);
    println!("GET METHOD: (AFTER SWAP): {:?}", new_list.get());
    println!("INDEX_OF_MIN METHOD: {}", new_list.index_of_min());
}

