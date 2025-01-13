#[macro_export]
macro_rules! hashset {
    ($($element:expr),*) => {{
        const COUNT: usize = $crate::count![@COUNT; $($element),*];

        #[allow(unused_mut)]
        let mut hm = ::std::collections::HashSet::with_capacity(COUNT);
        $(hm.insert($element);)*
        hm
    }};
    ($($element:expr,)*) => {{
        $crate::hashset![$($element),*]
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
fn expty_hashset() {
    let x: std::collections::HashSet<u32> = hashset![];
    assert!(x.is_empty());
}

#[test]
fn single_hashset() {
    let x = hashset![42];
    assert_eq!(x.len(), 1);
    assert!(x.contains(&42));
}

#[test]
fn double_hashset() {
    let x = hashset![42, 43];
    assert_eq!(x.len(), 2);
    assert!(x.contains(&42));
    assert!(x.contains(&43));
}

#[test]
fn duplicate_hashset() {
    let x = hashset![42, 34, 42, 42, 42];
    assert_eq!(x.len(), 2);
    assert!(x.contains(&42));
    assert!(x.contains(&34));
}

#[test]
fn trainling_hashset() {
    let x = hashset![42, 43,];
    assert_eq!(x.len(), 2);
    assert!(x.contains(&42));
    assert!(x.contains(&43));
}
