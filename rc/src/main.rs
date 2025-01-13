// Rc implementation simplified
// And why it's not Send

use std::ops::Deref;

struct Rc<T> {
    inner: *mut Inner<T>,
}

struct Inner<T> {
    value: T,
    count: usize,
}

impl<T> Rc<T> {
    pub fn new(value: T) -> Self {
        Self {
            inner: Box::into_raw(Box::new(Inner { count: 1, value })),
        }
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        unsafe { &mut *self.inner }.count += 1;
        Self { inner: self.inner }
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let count = &mut unsafe { &mut *self.inner }.count;
        if *count == 1 {
            let _ = unsafe { Box::from_raw(self.inner) };
        } else {
            *count -= 1;
        }
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &unsafe { &*self.inner }.value
    }
}

fn main() {
    let x = Rc::new(42);
    let y = x.clone();
    std::thread::spawn(move || {
        drop(y);
    });
}
