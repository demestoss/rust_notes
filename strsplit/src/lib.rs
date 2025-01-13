//! Docs?
#![warn(missing_debug_implementations, missing_docs)]

#[derive(Debug)]
pub struct StrSplit<'a, D> {
    remainder: Option<&'a str>,
    delimeter: D,
}

impl<'a, D> StrSplit<'a, D> {
    pub fn new(haystack: &'a str, delimeter: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimeter,
        }
    }
}

pub trait Delimeter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl<'a, D> Iterator for StrSplit<'a, D>
where
    D: Delimeter,
{
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;

        if let Some((delim_start, delim_end)) = self.delimeter.find_next(remainder) {
            let until_delim = &remainder[..delim_start];
            *remainder = &remainder[delim_end..];
            Some(until_delim)
        } else {
            self.remainder.take()
        }
    }
}

impl Delimeter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}

impl Delimeter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
            .find(|(_, c)| c == self)
            .map(|(start, _)| (start, start + self.len_utf8()))
    }
}

pub fn until_char(s: &str, c: char) -> &str {
    StrSplit::new(s, c)
        .next()
        .expect("StrSplit always gives at least on result")
}

#[test]
fn until_char_test() {
    assert_eq!(until_char("hello world", 'o'), "hell");
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters = StrSplit::new(haystack, " ").collect::<Vec<_>>();
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn tail() {
    let haystack = "a b c d ";
    let letters = StrSplit::new(haystack, " ").collect::<Vec<_>>();
    assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
}
