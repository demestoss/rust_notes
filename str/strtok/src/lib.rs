use delimiter::Delimiter;

pub fn strtok<'s>(s: &'_ mut &'s str, delimiter: impl Delimiter) -> &'s str {
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

struct TouchDrop<T: std::fmt::Debug>(T);

impl<T: std::fmt::Debug> Drop for TouchDrop<T> {
    fn drop(&mut self) {
        println!("{:?}", self.0);
    }
}

fn main() {
    let x = String::new();
    let z = vec![&x];
    // let z = vec![TouchDrop(&x)];
    drop(x);
    // drop(z);
}

use std::marker::PhantomData;
// involves drop check because it assumes that T will be dropped with the struct
struct Deserializer<T> {
    // some fields
    _t: PhantomData<T>,
}
// do not own the value, no drop check
// covariance over the T
struct Deserializer<T> {
    // some fields
    _t1: PhantomData<fn() -> T>,
    // not Send and Sync, so prev is preferable
    _t2: PhantomData<*const T>,
}
// contravariance over the T
struct Deserializer<T> {
    // some fields
    _t: PhantomData<fn(T)>,
}
// invariance over the T
struct Deserializer<T> {
    // some fields
    _t1: PhantomData<fn(T) -> T>,
    _t2: PhantomData<*mut T>,
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

/*
fn main() {
    let s = String::new();
    let x: &'static str = "hello";
    let mut y = &*s;
    // We can assign variable with a static lifetime here
    // it's valid for any context without static lifetime because it basically outlives any other
    // it can be done because of _Variance_
    //
    // Specifically it's called _Covariance_
    y = x;
}
*/

// T: U
// T subtype of U if T is at least as useful as U
//
// 'static: 'a
//
//
// covariance
//  foo(&'a str) {}
//  let x: &'a str
//
//  You can provide suptype of 'a
//  foo(&'a str)
//  foo(&'static str)
//
// contravariance
//
//  fn foo(bar: Fn(&'a str) -> ()) {
//      bar("" /* 'a */)
//  }
//
//  This one is not possible, because we are making it stricter
//  We can provide types that are less useful - contravariance
//  But providing types that are more useful will make it stricter
//  foo(fn(&'static str) {})
//
//  &'static str     // more useful
//  &'a str
//
//  'static <: 'a
//  &'static T <: &'a T
//
//  Fn(&'static str)
//  Fn(&'a str)      // more usefull
//
//  'static <: 'a
//  Fn(&'a T) <: Fn(&'static T)
//
// invariance
//
//  You should provide exactly the same type, not more or less useful
//
// fn foo(s: &mut &'a str, x: &'a str) {
//    *s = x;
// }
//
// let mut s: &'static str = "hello";
// let z = String::new();
// // should not compile
// // we are providing z with the lifetime that is less than s
// // and putting it inside z and than drop, so that should be compile error
// // that's why mut refs are invariant in T
// foo(&mut s, &z);
// drop(z);
// println!("{s}");

pub fn bar() {
    let mut x = true;
    let mut y /* &'y mut bool */ = &mut x;

    let z = Box::new(true);
    let z: &'static mut bool = Box::leak(z);

    // mut references covariant in their lifetime
    // but invariant in the T
    y = z; // &'y mut bool = &'static mut bool

    // ignore
    drop(y);
}
