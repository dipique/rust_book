mod type_alias;

pub fn run() {
    type_alias::run();

    // infinite loop
    // never_type();
}

fn never_type() -> ! {
    // this would not compile because it returns a unit type
    //println!("I can never return a type");

    loop {
        // This compiles, but it cannot break because that would
        // also return something
    }

    // this is used for match arms that don't return a value such
    // as "continue" or "panic!()"
}