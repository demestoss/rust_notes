use crate::Sorter;

pub struct BubbleSort;

impl Sorter for BubbleSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        let mut swapped = true;
        while swapped {
            swapped = false;
            for i in 1..slice.len() {
                if slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    swapped = true;
                }
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
        BubbleSort.sort(&mut x);
        assert_eq!(x, vec![1, 2, 3, 4]);
    }

    #[test]
    fn odd_length() {
        let mut x = vec![3, 4, 2, 1, 5];
        BubbleSort.sort(&mut x);
        assert_eq!(x, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn empty() {
        let mut x: Vec<()> = vec![];
        BubbleSort.sort(&mut x);
        assert_eq!(x, vec![]);
    }
}
