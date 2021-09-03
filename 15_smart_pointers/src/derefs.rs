use std::ops::Deref;

pub fn derefs() {
    basic_deref();
    box_deref();
    using_mybox();

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

pub fn basic_deref() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

pub fn box_deref() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

pub fn using_mybox() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

struct MyBox<T>(T); // tuple struct

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

// deref coercion works in 3 cases:
// - from &T     to     &U when T: Deref   <Target=U>
// - from &mut T to &mut U when T: DerefMut<Target=U>
// - from &mut T to     &U when T: Deref   <Target=U>