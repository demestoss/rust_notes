pub trait IteratorExt: Iterator + Sized {
    fn my_flat_map<F, U>(self, f: F) -> FlatMap<Self, F, U>
    where
        F: FnMut(Self::Item) -> U,
        U: IntoIterator;
}

impl<T> IteratorExt for T
where
    T: Iterator + Sized,
{
    fn my_flat_map<F, U>(self, f: F) -> FlatMap<Self, F, U>
    where
        F: FnMut(Self::Item) -> U,
        U: IntoIterator,
    {
        FlatMap::new(self, f)
    }
}

pub fn flat_map<I, F, U>(iter: I, f: F) -> FlatMap<I::IntoIter, F, U>
where
    I: IntoIterator,
    F: FnMut(I::Item) -> U,
    U: IntoIterator,
{
    FlatMap::new(iter.into_iter(), f)
}

pub struct FlatMap<O, F, U>
where
    O: Iterator,
    F: FnMut(O::Item) -> U,
    U: IntoIterator,
{
    outer: O,
    map_fn: F,
    inner: Option<U::IntoIter>,
}

impl<O, F, U> FlatMap<O, F, U>
where
    O: Iterator,
    F: FnMut(O::Item) -> U,
    U: IntoIterator,
{
    pub fn new(iter: O, f: F) -> Self {
        Self {
            outer: iter,
            map_fn: f,
            inner: None,
        }
    }
}

impl<O, F, U> Iterator for FlatMap<O, F, U>
where
    O: Iterator,
    F: FnMut(O::Item) -> U,
    U: IntoIterator,
{
    type Item = U::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(inner_iter) = &mut self.inner {
                if let Some(i) = inner_iter.next() {
                    return Some(i);
                }
                self.inner = None;
            }
            let next_outer_item = self.outer.next()?;
            let next_inner = (self.map_fn)(next_outer_item);
            self.inner = Some(next_inner.into_iter());
        }
    }
}

#[test]
fn one_count() {
    let x = flat_map(vec!["abc"], |i| i.chars());
    assert_eq!(x.count(), 3);
}

#[test]
fn one() {
    let mut x = flat_map(vec!["abc"], |i| i.chars());
    assert_eq!(x.next(), Some('a'));
    assert_eq!(x.next(), Some('b'));
    assert_eq!(x.next(), Some('c'));
    assert_eq!(x.next(), None);
}

#[test]
fn two() {
    let mut x = flat_map(vec!["abc", "def"], |i| i.chars());
    assert_eq!(x.next(), Some('a'));
    assert_eq!(x.next(), Some('b'));
    assert_eq!(x.next(), Some('c'));
    assert_eq!(x.next(), Some('d'));
    assert_eq!(x.next(), Some('e'));
    assert_eq!(x.next(), Some('f'));
}

#[test]
fn four_collect() {
    let x = flat_map(vec!["abc", "def", "hjk", "qwert"], |i| i.chars());
    assert_eq!(x.collect::<String>(), "abcdefhjkqwert");
}

#[test]
fn extension() {
    let x = vec!["abc", "def", "hjk", "qwert"]
        .into_iter()
        .my_flat_map(|i| i.chars());
    assert_eq!(x.collect::<String>(), "abcdefhjkqwert");
}
