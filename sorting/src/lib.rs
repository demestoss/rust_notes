pub trait Sorter {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord;
}

pub fn sort<T, S>(slice: &mut [T])
where
    T: Ord,
    S: Sorter,
{
    S::sort(slice)
}

type VecIntoIter<T> = std::vec::IntoIter<T>;

pub trait IteratorExt: Iterator + Sized {
    /// Like _sort_ but ton sorted
    fn sorted<S>(&mut self) -> VecIntoIter<Self::Item>
    where
        Self::Item: Ord,
        S: Sorter;
}

impl<T> IteratorExt for T
where
    T: Iterator + Sized,
    T::Item: Ord,
{
    fn sorted<S>(&mut self) -> VecIntoIter<Self::Item>
    where
        S: Sorter,
    {
        let mut vec = Vec::from_iter(self);
        S::sort(&mut vec);
        vec.into_iter()
    }
}

mod bubblesort;

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::assert_equal;

    struct StdSorter;
    impl Sorter for StdSorter {
        fn sort<T>(slice: &mut [T])
        where
            T: Ord,
        {
            slice.sort();
        }
    }

    #[test]
    fn std_works() {
        let x = [3, 4, 2, 1];
        let x = x.iter().sorted::<StdSorter>();
        assert_equal(x, &[1, 2, 3, 4]);
    }
}
