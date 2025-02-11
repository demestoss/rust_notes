use crate::Sorter;

pub struct MergeSort;

fn mergesort<T: Ord>(slice: &mut [T]) {
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

    let mid = slice.len() / 2;

    mergesort(&mut slice[..mid]);
    mergesort(&mut slice[mid..]);

    merge(slice, mid);
}

fn merge<T: Ord>(slice: &mut [T], mid: usize) {
    let mut mid = mid;
    let mut left = 0;
    let mut right = mid;

    // Looking at slice as a 2 slice with the boundary [| left - mid | | mid - right |]
    // Either moving until the left will be empty [sorted | mid - right |] => [sorted]
    // Or moving the mid pointer as well so the right will be empty at some point
    while left <= mid && right < slice.len() {
        if slice[left] <= slice[right] {
            left += 1;
        } else {
            slice[left..=right].rotate_right(1);
            left += 1;
            mid += 1;
            right += 1;
        }
    }
}

impl Sorter for MergeSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        mergesort(slice);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let mut x = vec![3, 4, 2, 1];
        MergeSort.sort(&mut x);
        assert_eq!(x, vec![1, 2, 3, 4]);
    }

    #[test]
    fn odd_length() {
        let mut x = vec![3, 4, 2, 1, 5];
        MergeSort.sort(&mut x);
        assert_eq!(x, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn long_length() {
        let mut x = vec![3, 4, 2, 1, 5, 12, 8];
        MergeSort.sort(&mut x);
        assert_eq!(x, vec![1, 2, 3, 4, 5, 8, 12]);
    }

    #[test]
    fn empty() {
        let mut x: Vec<()> = vec![];
        MergeSort.sort(&mut x);
        assert_eq!(x, vec![]);
    }
}
