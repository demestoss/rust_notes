use crate::Sorter;

pub struct InsertionSort {
    smart: bool,
}

impl Sorter for InsertionSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        // [sorted | not sorted]
        for unsorted in 1..slice.len() {
            // slice[unsorted..] is not sorted
            // take slice[unsorted] and place in sorted location is slice[..=unsorted]
            if !self.smart {
                let mut i = unsorted;
                while i > 0 && slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    i -= 1;
                }
            } else {
                // use binary search to find index
                // then move current unsorted to the right position
                let i = match slice[..unsorted].binary_search(&slice[unsorted]) {
                    Ok(i) => i,
                    Err(i) => i,
                };
                slice[i..=unsorted].rotate_right(1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dumb_works() {
        let mut x = vec![3, 4, 2, 1];
        InsertionSort { smart: false }.sort(&mut x);
        assert_eq!(x, vec![1, 2, 3, 4]);
    }

    #[test]
    fn dumb_odd_length() {
        let mut x = vec![3, 4, 2, 1, 5];
        InsertionSort { smart: false }.sort(&mut x);
        assert_eq!(x, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn dumb_empty() {
        let mut x: Vec<()> = vec![];
        InsertionSort { smart: false }.sort(&mut x);
        assert_eq!(x, vec![]);
    }

    #[test]
    fn smart_works() {
        let mut x = vec![3, 4, 2, 1];
        InsertionSort { smart: true }.sort(&mut x);
        assert_eq!(x, vec![1, 2, 3, 4]);
    }

    #[test]
    fn smart_odd_length() {
        let mut x = vec![3, 4, 2, 1, 5];
        InsertionSort { smart: true }.sort(&mut x);
        assert_eq!(x, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn smart_empty() {
        let mut x: Vec<()> = vec![];
        InsertionSort { smart: true }.sort(&mut x);
        assert_eq!(x, vec![]);
    }
}
