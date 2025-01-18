use delimiter::Delimiter;

pub fn strtok<'a>(s: &'a mut &'a str, delimiter: impl Delimiter) -> &'a str {
    if let Some((delim_start, delim_end)) = delimiter.find_next(s) {
        let prefix = &s[..delim_start];
        let suffix = &s[delim_end..];
        *s = suffix;
        prefix
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut x = "hello world";
        let hello = strtok(&mut x, ' ');
        assert_eq!(hello, "hello");
        assert_eq!(x, "world");
    }
}
