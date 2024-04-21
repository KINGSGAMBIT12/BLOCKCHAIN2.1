pub trait Sortable {
    fn compare(&self, other: &Self) -> std::cmp::Ordering;
}

pub mod sorting {
    use super::Sortable;

    pub fn quick_sort<T: Sortable>(arr: &mut [T]) {
        if arr.len() <= 1 {
            return;
        }
        let pivot = partition(arr);
        quick_sort(&mut arr[..pivot]);
        quick_sort(&mut arr[pivot + 1..]);
    }

    fn partition<T: Sortable>(arr: &mut [T]) -> usize {
        let pivot_index = arr.len() / 2;
        arr.swap(pivot_index, arr.len() - 1);
        let mut i = 0;
        for j in 0..arr.len() - 1 {
            if arr[j].compare(&arr[arr.len() - 1]) == std::cmp::Ordering::Less {
                arr.swap(i, j);
                i += 1;
            }
        }
        arr.swap(i, arr.len() - 1);
        i
    }

    pub fn selection_sort<T: Sortable>(arr: &mut [T]) {
        for i in 0..arr.len() {
            let mut min_index = i;
            for j in i + 1..arr.len() {
                if arr[j].compare(&arr[min_index]) == std::cmp::Ordering::Less {
                    min_index = j;
                }
            }
            if min_index != i {
                arr.swap(i, min_index);
            }
        }
    }

    pub fn insertion_sort<T: Sortable>(arr: &mut [T]) {
        for i in 1..arr.len() {
            let mut j = i;
            while j > 0 && arr[j].compare(&arr[j - 1]) == std::cmp::Ordering::Less {
                arr.swap(j, j - 1);
                j -= 1;
            }
        }
    }

pub fn merge_sort<T: Sortable + Clone + Copy>(arr: &mut [T]) {
    let mid = arr.len() / 2;
    if mid == 0 {
        return;
    }
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);

    let mut merged = Vec::with_capacity(arr.len());
    let (mut left, mut right) = (0, mid);

    while left < mid && right < arr.len() {
        if arr[left].compare(&arr[right]) == std::cmp::Ordering::Less {
            merged.push(arr[left].clone());
            left += 1;
        } else {
            merged.push(arr[right].clone());
            right += 1;
        }
    }

    while left < mid {
        merged.push(arr[left].clone());
        left += 1;
    }
    while right < arr.len() {
        merged.push(arr[right].clone());
        right += 1;
    }

    arr.copy_from_slice(&merged);
}

}
impl Sortable for i32 {
    fn compare(&self, other: &Self) -> std::cmp::Ordering {
        self.cmp(other)
    }
}
pub use sorting::*;
