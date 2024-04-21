##Sorting Library
##Overview
This is a Rust library crate that provides sorting algorithms such as quicksort, selection sort, insertion sort, and merge sort for various types of objects.

##Installation
To use this library, simply add it as a dependency in your Cargo.toml file:

[dependencies]
sorting_library = { git = "https://github.com/KINGSGAMBIT12/BLOCKCHAIN2.1.git" }
##Usage
Here's an example demonstrating how to use the sorting algorithms provided by this library:

use sorting_library::{Sortable, quick_sort, selection_sort, insertion_sort, merge_sort};

fn main() {
    let mut nums = vec![5, 3, 8, 1, 6];
    
    // Quicksort
    quick_sort(&mut nums);
    println!("Quicksort: {:?}", nums);
    
    // Selection Sort
    selection_sort(&mut nums);
    println!("Selection Sort: {:?}", nums);
    
    // Insertion Sort
    insertion_sort(&mut nums);
    println!("Insertion Sort: {:?}", nums);
    
    // Merge Sort
    merge_sort(&mut nums);
    println!("Merge Sort: {:?}", nums);
}
This code sorts a vector of numbers using different sorting algorithms provided by the sorting_library crate.

