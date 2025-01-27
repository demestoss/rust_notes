use std::sync::Arc;

pub struct ImmutableList<T> {
    head: Link<T>,
}

type Link<T> = Option<Arc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> ImmutableList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn prepend(&self, elem: T) -> ImmutableList<T> {
        ImmutableList {
            head: Some(Arc::new(Node {
                elem,
                next: self.head.clone(),
            })),
        }
    }

    pub fn tail(&self) -> ImmutableList<T> {
        ImmutableList {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
}

impl<T> Default for ImmutableList<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> ImmutableList<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

impl<T> Drop for ImmutableList<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Arc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::ImmutableList;

    #[test]
    fn basics() {
        let list = ImmutableList::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn iter() {
        let list = ImmutableList::new().prepend(1).prepend(2).prepend(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}
