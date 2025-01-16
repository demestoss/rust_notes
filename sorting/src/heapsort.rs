use crate::Sorter;

pub struct HeapSort;

impl Sorter for HeapSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
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
