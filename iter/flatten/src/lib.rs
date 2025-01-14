pub trait IteratorExt: Iterator + Sized {
    fn our_flatten(self) -> Flatten<Self>
    where
        Self::Item: IntoIterator;
}

impl<T> IteratorExt for T
where
    T: Iterator,
{
    fn our_flatten(self) -> Flatten<Self>
    where
        Self::Item: IntoIterator,
    {
        flatten(self)
    }
}

pub fn flatten<I>(iter: I) -> Flatten<I::IntoIter>
where
    I: IntoIterator,
    I::Item: IntoIterator,
{
    Flatten::new(iter.into_iter())
}

pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    front_iter: Option<<O::Item as IntoIterator>::IntoIter>,
    back_iter: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    pub fn new(iter: O) -> Self {
        Self {
            outer: iter,
            front_iter: None,
            back_iter: None,
        }
    }
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(inner_iter) = self.front_iter.as_mut() {
                if let Some(i) = inner_iter.next() {
                    return Some(i);
                }
                self.front_iter = None;
            }

            if let Some(next_inner) = self.outer.next() {
                let next_inner_iter = next_inner.into_iter();
                self.front_iter = Some(next_inner_iter);
            } else {
                return self.back_iter.as_mut()?.next();
            }
        }
    }
}

impl<O> DoubleEndedIterator for Flatten<O>
where
    O: DoubleEndedIterator,
    O::Item: IntoIterator,
    <O::Item as IntoIterator>::IntoIter: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(inner_iter) = self.back_iter.as_mut() {
                if let Some(i) = inner_iter.next_back() {
                    return Some(i);
                }
                self.back_iter = None;
            }

            if let Some(next_inner) = self.outer.next_back() {
                let next_inner_iter = next_inner.into_iter();
                self.back_iter = Some(next_inner_iter);
            } else {
                return self.front_iter.as_mut()?.next_back();
            }
        }
    }
}

#[test]
fn empty() {
    assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(), 0);
}

#[test]
fn empty_wide() {
    let x: Vec<Vec<()>> = vec![vec![], vec![], vec![]];
    assert_eq!(flatten(x).count(), 0);
}

#[test]
fn once() {
    assert_eq!(flatten(std::iter::once(vec!["a"])).count(), 1);
}

#[test]
fn two() {
    assert_eq!(flatten(std::iter::once(vec!["a", "b"])).count(), 2);
}

#[test]
fn two_wide() {
    assert_eq!(flatten(vec![vec!["a"], vec!["b"]]).count(), 2);
}

#[test]
fn reverse() {
    assert_eq!(
        flatten(std::iter::once(vec!["a", "b"]))
            .rev()
            .collect::<Vec<_>>(),
        vec!["b", "a"]
    );
}

#[test]
fn reverse_wide() {
    assert_eq!(
        flatten(vec![vec!["a"], vec!["b"]])
            .rev()
            .collect::<Vec<_>>(),
        vec!["b", "a"]
    );
}

#[test]
fn both_ends() {
    let mut v = flatten(vec![vec!["a", "b"], vec!["c", "d"]]);
    assert_eq!(v.next(), Some("a"));
    assert_eq!(v.next_back(), Some("d"));
    assert_eq!(v.next(), Some("b"));
    assert_eq!(v.next_back(), Some("c"));
    assert_eq!(v.next(), None);
    assert_eq!(v.next_back(), None);
}

#[test]
fn both_ends_one_back() {
    let mut v = flatten(vec![vec!["a", "b"], vec!["c", "d"]]);
    assert_eq!(v.next(), Some("a"));
    assert_eq!(v.next_back(), Some("d"));
    assert_eq!(v.next(), Some("b"));
    assert_eq!(v.next(), Some("c"));
    assert_eq!(v.next(), None);
    assert_eq!(v.next_back(), None);
}

#[test]
fn both_ends_one_front() {
    let mut v = flatten(vec![vec!["a", "b"], vec!["c", "d"]]);
    assert_eq!(v.next(), Some("a"));
    assert_eq!(v.next_back(), Some("d"));
    assert_eq!(v.next_back(), Some("c"));
    assert_eq!(v.next_back(), Some("b"));
    assert_eq!(v.next(), None);
    assert_eq!(v.next_back(), None);
}

#[test]
fn infinite() {
    let mut x = flatten((0..).map(|i| 0..i));
    assert_eq!(x.next(), Some(0));
    assert_eq!(x.next(), Some(0));
    assert_eq!(x.next(), Some(1));
}

#[test]
fn deep() {
    assert_eq!(flatten(flatten(vec![vec![vec![0, 1]]])).count(), 2);
}

#[test]
fn ext() {
    assert_eq!(vec![vec![0, 1]].into_iter().our_flatten().count(), 2);
}
