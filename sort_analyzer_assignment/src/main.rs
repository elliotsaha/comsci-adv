use crate::sort::{FileSorter, SortMethods};
pub mod sort;
pub mod utils;

fn main() {
    let files = [
        "random-values",
        "reversed-values",
        "nearly-sorted-values",
        "few-unique-values",
    ];

    let sort_order = ["Bubble Sort", "Selection Sort", "Insertion Sort"];

    let sort_methods = [
        FileSorter::bubble_sort,
        FileSorter::selection_sort,
        FileSorter::insertion_sort,
    ];

    for i in 0..3 {
        println!("{}", sort_order[i]);
        for file_name in files {
            let mut total = 0.0;

            for _ in 0..5 {
                let iter = sort_methods[i](file_name);
                total += iter;
            }

            println!("{file_name}: {}", total / 5.0);
        }
    }
}
