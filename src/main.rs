use sorting_library::{Sortable, quick_sort, selection_sort, insertion_sort, merge_sort};

fn main() {
    let mut nums = vec![5, 3, 8, 1, 6];
    quick_sort(&mut nums);
    println!("Quicksort: {:?}", nums);
    selection_sort(&mut nums);
    println!("Selection Sort: {:?}", nums);
    insertion_sort(&mut nums);
    println!("Insertion Sort: {:?}", nums);
    merge_sort(&mut nums);
    println!("Merge Sort: {:?}", nums);
}
