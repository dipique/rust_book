fn main() {
    borrow_string_for_length();
    mutable_reference();
    only_one_mut_ref();
    simultaneous_mutable_references();
    //dangling_references();
}

// Rules:
// 1) There can either be one mutable or unlimited immutable references to
//    a value at any one time.
// 2) References must always be valid.

///////////////////////////////////////////////

// Rust won't allow the dangle function it creates
// a reference to something that immediately goes
// out of scope.

// fn dangling_references() {
//     dangle();
// }

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

///////////////////////////////////////////////

fn simultaneous_mutable_references() {
    let mut s = String::from("Simultaneous mutable references");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here
    let r2 = &mut s;
    r2.push_str(" - appended");
    println!("{}", s);

    // // we cannot have a mutable reference while the value is borrowed as immutable
    // let mut s = String::from("SMR error example");
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // big problem!

    // println!("{} {} {}", r1, r2, r3);
}

///////////////////////////////////////////////

fn only_one_mut_ref() {
    let mut s = String::from("Only one mut ref");
    println!("{}", s);
    let r1 = &mut s;
    //println!("{}", s); // can't do this because it is borrowed
    r1.push_str(" - appended");
    println!("{}", r1);
    let r2 = &mut s; // I don't understand why this works exactly --- I think this works because the scope of a reference starts
                     // where it is introduced and ends on the last line where it is used, so r1 goes out of scope before this.
                     // This can sort of be confirmed with the sample code at the bottom, where removing the println resolves the
                     // error because r1 would not stay in scope.
    // println!("{}", r1); // can't do this because creating r2 "unborrowed" r1
    r2.push_str(" - twice!");
    println!("{}", r2);
    
    // it seems like you can create two mutable references as long as the first one
    // is used to mutate the value before the second one is created -- seems like a
    // trap

    //this is the sample code, which does not work because there are two mutable references
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
   // println!("{}, {}", r1, r2);
}

///////////////////////////////////////////////

fn mutable_reference() {
    let mut s = String::from("Mutable reference");
    change(&mut s);
    println!("{}", s);
}

fn change(s: &mut String) {
    s.push_str(" - I have appended!");
}
///////////////////////////////////////////////

fn borrow_string_for_length() {
    let s = String::from("Borrow string for length");
    println!("The string length is {}", calculate_length(&s));
    println!("And the string is: '{}'", s);
}

fn calculate_length(s: &String) -> usize {
    // s.push_str(" add me! "); // this a reference and therefore cannot be mutated
    s.len()
} // s is a pointer to the stack, so when it goes out of scope, the actual string is not freed

///////////////////////////////////////////////
