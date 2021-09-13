use std::fmt;

// this pattern allows us to implement a trait on a type where either the type nor the
// trait are local to the crate which is usually not allowed; the downside is that basically
// your new type ONLY has that trait unless you implement all the other methods etc. manually
// for your wrapper type

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

pub fn run() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}