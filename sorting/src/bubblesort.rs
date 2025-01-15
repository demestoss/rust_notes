use crate::Sorter;

pub struct Bubblesort;

impl Sorter for Bubblesort {
    fn sort<T>(slice: &mut [T])
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
    use crate::IteratorExt;

    #[test]
    fn works() {
        let x = [3, 4, 2, 1];
        let x = x.iter().sorted::<Bubblesort>();
        itertools::assert_equal(x, &[1, 2, 3, 4]);
    }

    #[test]
    fn odd_length() {
        let x = [3, 4, 2, 1, 5];
        let x = x.iter().sorted::<Bubblesort>();
        itertools::assert_equal(x, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn empty() {
        let x: Vec<()> = vec![];
        let x = x.iter().sorted::<Bubblesort>();
        itertools::assert_equal(x, &[]);
    }
}
