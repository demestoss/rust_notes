#[macro_export]
macro_rules! avec {
    ($($element:expr),*) => {{
        // check that count is const
        const COUNT: usize = $crate::count![@COUNT; $($element),*];

        #[allow(unused_mut)]
        let mut v = Vec::with_capacity(COUNT);
        $(v.push($element);)*
        v
    }};
    ($($element:expr,)*) => {{
        $crate::avec![$($element),*]
    }};
    ($element:expr; $count:expr) => {{
        let mut v = Vec::new();
        // v.extend(::core::iter::repeat($element).take(count));
        v.resize($count, $element);
        v
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! count {
    (@COUNT; $($element:expr),*) => {
        <[()]>::len(&[$($crate::count![@SUBST; $element]),*])
    };
    (@SUBST; $_element:expr) => {
        ()
    };
}

#[test]
fn empty_vec() {
    let x: Vec<u32> = avec![];
    assert!(x.is_empty());
}

#[test]
fn single() {
    let x = avec![42];
    assert_eq!(x.len(), 1);
    assert_eq!(x[0], 42);
}

#[test]
fn double() {
    let x: Vec<u32> = avec![42, 43];
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 43);
}

#[test]
fn trainling() {
    let x = avec![42, 43,];
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 43);
}

#[test]
fn clone_3() {
    let x = avec![42; 3];
    assert_eq!(x.len(), 3);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 42);
    assert_eq!(x[2], 42);
}

#[test]
fn clone_2_non_literal() {
    let mut y = Some(42);
    let x = avec![y.take().unwrap(); 2];
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 42);
}

/// ```compile_fail
/// let x = vecmac::avec![42; "foo"];
/// ```
#[allow(dead_code)]
struct CompileFailTestInvalidCount;

/// ```compile_fail
/// let x: Vec<u32> = vecmac::avec![,];
/// ```
#[allow(dead_code)]
struct CompileFailTestComaOnly;
