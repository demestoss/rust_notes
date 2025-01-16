use crate::Sorter;

pub struct SelectionSort;

impl Sorter for SelectionSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        // [sorted | not sorted]
        for unsorted in 0..slice.len() {
            let smallest_in_rest = slice[unsorted..]
                .iter()
                .enumerate()
                .min_by_key(|&(_, v)| v)
                .map(|(i, _)| i + unsorted)
                .expect("slice is non-empty");

            // or
            //
            // let mut smallest_in_rest = unsorted;
            // for i in (unsorted + 1)..slice.len() {
            //     if slice[i] < slice[smallest_in_rest] {
            //         smallest_in_rest = i;
            //     }
            // }

            if smallest_in_rest != unsorted {
                slice.swap(smallest_in_rest, unsorted);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let mut x = vec![3, 4, 2, 1];
        SelectionSort.sort(&mut x);
        assert_eq!(x, vec![1, 2, 3, 4]);
    }

    #[test]
    fn odd_length() {
        let mut x = vec![3, 4, 2, 1, 5];
        SelectionSort.sort(&mut x);
        assert_eq!(x, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn empty() {
        let mut x: Vec<()> = vec![];
        SelectionSort.sort(&mut x);
        assert_eq!(x, vec![]);
    }
}
