pub struct List {
   pub vector: Vec<isize> // guarenteed to have enough size for any architecture
}

pub trait ListOperations {
    fn get(&self) -> &Vec<isize>;
    fn contains(&self, item: isize) -> bool;
    fn index_of(&self, item: isize) -> isize;
    fn reverse(&self) -> Vec<isize>;
    fn swap(&mut self, idx1: usize, idx2: usize);
    fn index_of_min(&self) -> usize;
}

impl ListOperations for List {
    // returns self.vector
    fn get(&self) -> &Vec<isize> {
        &self.vector
    }
    
    // returns true if item is contained within vector
    fn contains(&self, item: isize) -> bool {
        let mut contains = false;
        for i in &self.vector {
            if *i == item {
                contains = true;
            }
        }
        contains
    }
    
    // returns first index of given vector int
    fn index_of(&self, item: isize) -> isize {
        for (idx, i) in self.vector.iter().map(|vec_item| (*vec_item) as isize).enumerate() {
            if i == item {
                return idx as isize;
            }
        }
        // returns -1 if index not found
        -1
    }
    
    // returns reversed copy of vector
    fn reverse(&self) -> Vec<isize> {
        let mut idx: isize = (self.vector.len() - 1) as isize;
        let mut reverse_vec = Vec::new(); 
        while idx >= 0 {
            reverse_vec.push(self.vector[idx as usize]);
            idx -= 1;
        }
        reverse_vec
    }
    
    // swaps two items in vector
    fn swap(&mut self, idx1: usize, idx2: usize) {
        // create temporary copies of each value being swapped
        let temp_idx1 = self.vector[idx1];
        let temp_idx2 = self.vector[idx2];
        // let temp copies equal vector values
        self.vector[idx1] = temp_idx2;
        self.vector[idx2] = temp_idx1;
    }
    
    // gets the index of the smallest number in vector
    fn index_of_min(&self) -> usize {
        let mut min_idx = 0;
        for (idx, i) in self.vector.iter().enumerate() {
            if *i < self.vector[min_idx] {
                min_idx = idx;
            }
        }
        min_idx
    }
}
