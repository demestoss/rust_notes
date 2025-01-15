use std::{marker::PhantomData, ops::Deref, ptr::NonNull};

use cell::Cell;

pub struct Rc<T> {
    inner: NonNull<RcInner<T>>,
    _marker: PhantomData<RcInner<T>>,
}

struct RcInner<T> {
    value: T,
    refcount: Cell<usize>,
}

impl<T> Rc<T> {
    pub fn new(value: T) -> Self {
        let inner = Box::new(RcInner {
            refcount: Cell::new(1),
            value,
        });
        Self {
            // SAFETY
            // Box does not gives us null pointer
            inner: unsafe { NonNull::new_unchecked(Box::into_raw(inner)) },
            _marker: PhantomData,
        }
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.refcount.get();
        inner.refcount.set(c + 1);
        Self {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // SAFETY
        // self.inner is a Box that is only deallocated when the last Rc goes away
        // We have an Rc, therefore the box was no deallocated so Deref is fine
        &unsafe { self.inner.as_ref() }.value
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.inner.as_ref() };
        let count = inner.refcount.get();
        if count == 1 {
            // SAFETY
            // We are the _only_ Rc left, and we are been dropped.
            // therefore, after us, there will be no Rc's left, and no references to T.
            let _ = unsafe { Box::from_raw(self.inner.as_ptr()) };
        } else {
            // There are others Rc, so don't drop the Box
            inner.refcount.set(count - 1);
        }
    }
}

/// DropCheck
/// ```
/// struct Foo<'a, T: Default> {
///     v: &'a mut T,
/// }
///
/// impl<T: Default> Drop for Foo<'_, T> {
///     fn drop(&mut self) {
///         let _ = std::mem::replace(self.v, T::default());
///     }
/// }
/// ```
/// ```compile_fail
/// let (foo, t);
/// t = String::from("dsdsd");
/// foo = Foo { v: &mut t };
/// ```
/// By default Rust drops values in the reverse order, so t --> foo
/// But here Rust actually knows that the foo uses mut ref to t and
/// we trigger a mut method of t in Drop implementation, so
/// it should drop foo first and only after that t
///
/// This is DropCheck and we need to do the same for Rc so the compiler
/// would know that we have a reference to t and check Drop implementation
///
/// So this one should fail to compile
/// ```compile_fail
/// let (foo, mut t);
/// t = String::from("dsdsd");
/// foo = rc::Rc::new(Foo { v: &mut t });
/// ```
///
#[allow(dead_code)]
struct DropCheckShouldNotCompile;

/// ```compile_fail
/// let x = rc::Rc::new(42);
/// let y = x.clone();
/// std::thread::spawn(move || {
///     drop(y);
/// });
/// ```
#[allow(dead_code)]
struct ShouldNotCompile;
