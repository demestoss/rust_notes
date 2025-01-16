use crate::Sorter;

pub struct QuickSort;

fn quicksort<T: Ord>(slice: &mut [T]) {
    match slice.len() {
        0 | 1 => return,
        2 => {
            if slice[0] > slice[1] {
                slice.swap(0, 1);
            }
            return;
        }
        _ => {}
    }

    let (pivot, rest) = slice.split_first_mut().expect("slice is non-empty");

    let mut left = 0;
    let mut right = rest.len() - 1;

    while left <= right {
        if &rest[left] <= pivot {
            // already on the correct side
            left += 1;
        } else if &rest[right] > pivot {
            // right already on the correct side
            // avoid unnecessary swaps backs and forth
            right -= 1;
        } else {
            // move element to the right side
            rest.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    // place pivot at the end of the left
    slice.swap(0, left);

    quicksort(&mut slice[..left]);
    quicksort(&mut slice[left + 1..]);
}

impl Sorter for QuickSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        // [unsorted | pivot | unsorted]
        quicksort(slice);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let mut x = vec![3, 4, 2, 1];
        QuickSort.sort(&mut x);
        assert_eq!(x, vec![1, 2, 3, 4]);
    }

    #[test]
    fn odd_length() {
        let mut x = vec![3, 4, 2, 1, 5];
        QuickSort.sort(&mut x);
        assert_eq!(x, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn empty() {
        let mut x: Vec<()> = vec![];
        QuickSort.sort(&mut x);
        assert_eq!(x, vec![]);
    }
}
