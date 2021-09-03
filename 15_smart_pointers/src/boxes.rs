// For an enum value, Rust will reserve enough
// space to store the largest variant
enum List {
    Cons(i32, Box<List>), // box prevents recursive type from using infinite memory
    Nil,
}

use List::{Cons, Nil};

pub fn boxes() {
    // store an integer on the heap
    let b = Box::new(5);
    println!("b = {}", b);

    let list =
        Cons(1, Box::new(
            Cons(2, Box::new(
                Cons(3, Box::new(Nil))
            ))
        ));
}

