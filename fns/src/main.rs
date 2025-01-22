fn main() {
    // function item
    let mut x = bar::<u32>;
    // x = bar::<u16>;
    assert_eq!(0, std::mem::size_of_val(&x));

    // function pointers
    baz(bar::<u32>);
    baz(bar::<i32>);

    quox(bar::<u32>);
}

fn bar<T>() {}

fn baz(f: fn()) {
    println!("{}", std::mem::size_of_val(&f));
}

// impl<F> FnOnce() for F
// where
//     F: Fn(),
// {
//     fn call(self) {
//         Fn::call(&self)
//     }
// }
//
// impl<F> FnOnce() for F
// where
//     F: FnMut(),
// {
//     fn call(mut self) {
//         Fn::call(&mut self)
//     }
// }
//
// impl<F> FnMut() for F
// where
//     F: Fn(),
// {
//     fn call(&mut self) {
//         Fn::call(&*self)
//     }
// }

fn quox<F>(f: F)
where
    F: Fn(),
{
}
