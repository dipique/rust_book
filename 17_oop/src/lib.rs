pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

// when we use trait objects, rust has to use dynamic dispatch, which means
// it needs to emit code to identify the correct method at runtime. It also
// prevents certain optimizations such as code in-lining.

// a trait objet requires an "object-safe" trait, which has two rules:
// (1) The return type != self
//      because the type of "self" is forgotten and thus can't be accessed
// (2) There are no generic type parameters
//      same is true of generic type parameters

// an example of a trait that is NOT object-safe is Clone:
pub trait Clone {
    fn clone(&self) -> Self; // returns self, so NOT object safe
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
        println!("Draw Button");
    }
}