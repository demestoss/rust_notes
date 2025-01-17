use crate::Sorter;

pub struct HeapSort;

fn heapify<T: Ord>(slice: &mut [T]) {
    //
}

fn swap_on_place<T: Ord>(slice: &mut [T], parent_idx: usize) {
    let left_idx = parent_idx * 2 + 1;
    let right_idx = parent_idx * 2 + 2;

    if let Some(left) = slice.get(left_idx) {
        let current = &slice[parent_idx];
        if left > current {
            slice.swap(parent_idx, left_idx);
            swap_on_place(slice, left_idx);
        }
    }

    if let Some(right) = slice.get(right_idx) {
        let current = &slice[parent_idx];
        if right > current {
            slice.swap(parent_idx, right_idx);
            swap_on_place(slice, right_idx);
        }
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
            swap_on_place(&mut slice[0..last_index], 0);
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
