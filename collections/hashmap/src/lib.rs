#[macro_export]
macro_rules! hashmap {
    ($($key:expr => $value:expr),*) => {{
        const COUNT: usize = $crate::count![@COUNT; $($key),*];

        #[allow(unused_mut)]
        let mut hm = ::std::collections::HashMap::with_capacity(COUNT);
        $(hm.insert($key, $value);)*
        hm
    }};
    ($($key:expr => $value:expr,)*) => {{
        $crate::hashmap![$($key => $value),*]
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
fn empty_hashmap() {
    let x: std::collections::HashMap<u32, u32>  = hashmap!{};
    assert!(x.is_empty());
}

#[test]
fn single_hashmap() {
    let x = hashmap!{
        42 => "test"
    };
    assert_eq!(x.len(), 1);
    assert_eq!(x.get(&42), Some(&"test"));
}

#[test]
fn double_hashmap() {
     let x = hashmap!{
        42 => "test",
        43 => "mock"
    };
    assert_eq!(x.len(), 2);
    assert_eq!(x.get(&42), Some(&"test"));
    assert_eq!(x.get(&43), Some(&"mock"));

}

#[test]
fn trainling_hashmap() {
     let x = hashmap!{
        42 => "test",
        43 => "mock",
    };
    assert_eq!(x.len(), 2);
    assert_eq!(x.get(&42), Some(&"test"));
    assert_eq!(x.get(&43), Some(&"mock"));
}

