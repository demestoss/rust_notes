// doubly-linked-deque

use std::{marker::PhantomData, ptr::NonNull};

pub struct LinkedList<T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
    _boo: PhantomData<T>,
}

type Link<T> = Option<NonNull<Node<T>>>;

struct Node<T> {
    front: Link<T>,
    back: Link<T>,
    elem: T,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            front: None,
            back: None,
            len: 0,
            _boo: PhantomData,
        }
    }

    pub fn push_front(&mut self, elem: T) {
        // Safety: We get pointer from Box which could not be null
        let new = unsafe {
            NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                front: None,
                back: None,
                elem,
            })))
        };

        if let Some(old) = self.front {
            // Safety: We use pointer that was constructed before as not null
            unsafe {
                (*new.as_ptr()).back = Some(old);
            }
            // Safety: Pointer on the front is not null, it only can be constructed from Box
            unsafe {
                (*old.as_ptr()).front = Some(new);
            }
        } else {
            debug_assert!(self.back.is_none());
            debug_assert!(self.front.is_none());
            debug_assert_eq!(self.len, 0);
            self.back = Some(new);
        }

        self.front = Some(new);
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let node = self.front?;
        // Safety: Pointer on the front is not null, it only can be constructed from Box
        let boxed_node = unsafe { Box::from_raw(node.as_ptr()) };
        let result = boxed_node.elem;

        // Make the next node into the new front.
        self.front = boxed_node.back;
        if let Some(new) = self.front {
            // Safety: Pointer on the front is not null, it only can be constructed from Box
            unsafe {
                (*new.as_ptr()).front = None;
            }
        } else {
            debug_assert!(self.front.is_none());
            debug_assert!(self.len == 1);
            self.back = None;
        }

        self.len -= 1;
        Some(result)
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::LinkedList;

    #[test]
    fn test_basic_front() {
        let mut list = LinkedList::default();

        // Try to break an empty list
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);

        // Try to break a one item list
        list.push_front(10);
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);

        // Mess around
        list.push_front(10);
        assert_eq!(list.len(), 1);
        list.push_front(20);
        assert_eq!(list.len(), 2);
        list.push_front(30);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(30));
        assert_eq!(list.len(), 2);
        list.push_front(40);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(40));
        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_front(), Some(20));
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
    }
}
