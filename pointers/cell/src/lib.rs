use std::cell::UnsafeCell;

pub struct Cell<T> {
    // UnsafeCell is an only way in Rust to cast shared reference into exclusive reference
    inner: UnsafeCell<T>,
}

// UnsafeCell is not sync so this one is true
// impl<T> !Sync for Cell<T> {}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Self {
            inner: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        // SAFETY: we know no-one else concurrently mutation self.inner
        // SAFETY: we know we are not invalidating any references because we never give any out
        unsafe { *self.inner.get() = value };
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        // SAFETY: we know no-one else is modifying the value, since the only on thread can mutate
        // (because !Sync), and it's executing this function instead
        unsafe { *self.inner.get() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{sync::Arc, thread};

    // Why Cell is !Sync
    #[test]
    #[should_panic]
    fn bad_sync_safe_should_panic() {
        unsafe impl<T> Sync for Cell<T> {}

        let x = Arc::new(Cell::new(0));
        let x1 = x.clone();
        let x2 = x.clone();
        let t1 = thread::spawn(move || {
            for _ in 0..100000 {
                x1.set(x1.get() + 1);
            }
        });
        let t2 = thread::spawn(move || {
            for _ in 0..100000 {
                x2.set(x2.get() + 1);
            }
        });
        t1.join().unwrap();
        t2.join().unwrap();
        assert_eq!(x.get(), 200000)
    }
}
