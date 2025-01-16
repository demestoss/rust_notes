pub trait Sorter {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord;
}

pub struct StdSort;
impl Sorter for StdSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        slice.sort();
    }
}

mod bubblesort;
mod heapsort;
mod insertionsort;
mod mergesort;
mod quicksort;
mod selectionsort;

pub use bubblesort::BubbleSort;
pub use heapsort::HeapSort;
pub use insertionsort::InsertionSort;
pub use mergesort::MergeSort;
pub use quicksort::QuickSort;
pub use selectionsort::SelectionSort;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn std_works() {
        let mut x = vec![3, 4, 2, 1];
        StdSort.sort(&mut x);
        assert_eq!(x, vec![1, 2, 3, 4]);
    }
}
