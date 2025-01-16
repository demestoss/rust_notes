pub trait Sorter {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord;
}

mod bubblesort;
mod insertionsort;
mod selectionsort;
mod quicksort;

#[cfg(test)]
mod tests {
    use super::*;

    struct StdSort;
    impl Sorter for StdSort {
        fn sort<T>(&self, slice: &mut [T])
        where
            T: Ord,
        {
            slice.sort();
        }
    }

    #[test]
    fn std_works() {
        let mut x = vec![3, 4, 2, 1];
        StdSort.sort(&mut x);
        assert_eq!(x, vec![1, 2, 3, 4]);
    }
}
