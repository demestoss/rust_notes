use boks::Boks;

struct TouchDrop<T: std::fmt::Debug>(T);

impl<T: std::fmt::Debug> Drop for TouchDrop<T> {
    fn drop(&mut self) {
        println!("{:?}", self.0);
    }
}

fn main() {
    let x = 42;
    let b = Boks::new(x);
    println!("{}", *b);

    let mut y = 42;
    let b = Boks::new(&mut y);
    println!("{}", y);

    let mut y = 42;
    let b = Boks::new(TouchDrop(&mut y));
    println!("{}", y);

    let s = String::new();
    let mut box1 = Box::new(&*s);
    let box2: Box<&'static str> = Box::new("hello");
    box1 = box2;

    let s = String::new();
    let mut boks1 = Boks::new(&*s);
    let boks2: Boks<&'static str> = Boks::new("hello");
    boks1 = boks2;

}

