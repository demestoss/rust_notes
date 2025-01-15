use std::{borrow::Borrow, ops::Deref};

/// ```
/// fn escape<'a>(s: &'a str) -> Cow<'_, str> {
///     // ' => \'
///     // " => \"
///     // foo => foo
///     if already_escaped() {
///         Cow::Borrow(s)
///     } else {
///         let mut string = s.to_string();
///         // Do operrations to escaping
///         Cow::Owned(string)
///     }
/// }
/// ```
///
///
/// ```
/// impl String {
///     fn from_utf8_lossy(bytes: &[u8]) -> Cow<'_, str> {
///         if valid_utf8(bytes) {
///             Cow::Borrowed(bytes as &str)
///         } else {
///             let mut bts = Vec::from(bytes);
///             for bts {
///                 // Replace invalid UTF-8 chars
///             }
///             Cow::Owned(bts as String)
///         }
///     }
/// }
/// ```
///
pub enum Cow<'a, B: 'a>
where
    B: ToOwned + ?Sized,
{
    Borrowed(&'a B),
    Owned(B::Owned),
}

impl<B: ToOwned + ?Sized> Deref for Cow<'_, B> {
    type Target = B;
    fn deref(&self) -> &Self::Target {
        match *self {
            Self::Borrowed(b) => b,
            Self::Owned(ref o) => o.borrow(),
        }
    }
}

impl<B: ToOwned + ?Sized> Clone for Cow<'_, B> {
    fn clone(&self) -> Self {
        match *self {
            Self::Borrowed(b) => Self::Borrowed(b),
            Self::Owned(ref owned) => Self::Owned(owned.borrow().to_owned()),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        match (self, source) {
            (Self::Owned(ref mut dest), Self::Owned(ref source)) => {
                source.borrow().clone_into(dest);
            }
            (t, s) => *t = s.clone(),
        }
    }
}

impl<B: ToOwned + ?Sized> Cow<'_, B> {
    pub fn is_borrowed(&self) -> bool {
        match self {
            Self::Borrowed(_) => true,
            Self::Owned(_) => false,
        }
    }

    pub fn is_owned(&self) -> bool {
        !self.is_borrowed()
    }

    pub fn to_mut(&mut self) -> &mut B::Owned {
        match *self {
            Self::Borrowed(borrowed) => {
                *self = Self::Owned(borrowed.to_owned());
                self.to_mut()
            }
            Self::Owned(ref mut o) => o,
        }
    }

    pub fn into_owned(self) -> B::Owned {
        match self {
            Self::Borrowed(b) => b.to_owned(),
            Self::Owned(o) => o,
        }
    }
}
