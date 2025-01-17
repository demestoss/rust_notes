use crate::Sorter;

pub struct HeapSort;

fn heapify<T: Ord>(slice: &mut [T]) {
    // Going backwards through slice and swap elements is child is less than parent
    for i in 0..slice.len() {
        shift_elem(slice, slice.len() - 1 - i);
    }
}

// Shift element from the current index to the place where it belongs
// Checking left and right branches accordingly
fn shift_elem<T: Ord>(slice: &mut [T], current_idx: usize) {
    let left_idx = current_idx * 2 + 1;
    let right_idx = current_idx * 2 + 2;

    swap_on_place(slice, current_idx, left_idx);
    swap_on_place(slice, current_idx, right_idx);
}

// Swap if the current item is less than child
// Using Option type to cmp
fn swap_on_place<T: Ord>(slice: &mut [T], parent_idx: usize, next_idx: usize) {
    let current = slice.get(parent_idx);

    if slice.get(next_idx) > current {
        slice.swap(parent_idx, next_idx);
        shift_elem(slice, next_idx);
    }
}

impl Sorter for HeapSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        heapify(slice);

        for i in 1..slice.len() {
            let last_index = slice.len() - i;
            slice.swap(0, last_index);
            shift_elem(&mut slice[0..last_index], 0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let mut x = vec![3, 4, 2, 1];
        HeapSort.sort(&mut x);
        assert_eq!(x, vec![1, 2, 3, 4]);
    }

    #[test]
    fn odd_length() {
        let mut x = vec![3, 4, 2, 1, 5];
        HeapSort.sort(&mut x);
        assert_eq!(x, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn empty() {
        let mut x: Vec<()> = vec![];
        HeapSort.sort(&mut x);
        assert_eq!(x, vec![]);
    }
}
