#![feature(dropck_eyepatch)]
use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

pub struct Boks<T> {
    p: NonNull<T>,
    _t: PhantomData<T>,
}

unsafe impl<#[may_dangle] T> Drop for Boks<T> {
    fn drop(&mut self) {
        // Safety
        // p was constructed using Box and has not been freed otherwise since self still
        // exists
        drop(unsafe { Box::from_raw(self.p.as_ptr()) });
    }
}

impl<T> Deref for Boks<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // Safety
        // is valid since it was constructed through valid T, and turned into the pointer through
        // Box which creates aligned pointers, and hasn't been freed, since self is alive
        unsafe { self.p.as_ref() }
    }
}

impl<T> DerefMut for Boks<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety
        // Look at Deref impl
        // Also, since we have &mut self, no other references has been given out
        unsafe { self.p.as_mut() }
    }
}

impl<T> Boks<T> {
    pub fn new(x: T) -> Self {
        Self {
            // Safety
            // Box will now return null pointer
            p: unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(x))) },
            _t: PhantomData,
        }
    }
}
