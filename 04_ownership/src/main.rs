fn main() {
    str_ex();
    str_mv_ex();
    clone_ex();
    take_ownership();
    return_values_and_scope();
    using_a_tuple();
}

///////////////////////////////////////////////

fn using_a_tuple() {
    let s1 = String::from("Using a tuple");
    let (s2, len) = calculate_length(s1);
    println!("The size of '{}' is {}", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

///////////////////////////////////////////////

fn return_values_and_scope() {
    let s1 = gives_ownership(); // gets a string from the function
    let s2 = String::from("return values and scope");
    let s3 = takes_and_returns_ownership(s2);

    // println!("{}", s2); // s2 is no longer valid because it was passed into function
    println!("{}", s3); // but now s3 has it
}

fn gives_ownership() -> String {
    let s = String::from("gives ownership");
    s // ownership is given to the calling function 
}

fn takes_and_returns_ownership(some_string: String) -> String {
    println!("{}", some_string);
    some_string
}

///////////////////////////////////////////////

fn take_ownership() {
    let s = String::from("take ownership");
    takes_ownership(s);
    //println!("{}", s); // invalid because this function took ownership

    let x = 5;
    makes_copy(x);
    println!("{}", x); // this works because i32 has the Copy trait
} // x goes out of scope, but s is already out of scope by this point

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string now goes out of scope and the memory is freed

fn makes_copy(some_integer: i32){
    println!("{}", some_integer);
} // some_integer goes out of scope, but there is no heap data so nothing special happens

///////////////////////////////////////////////

fn clone_ex() {
    let s1 = String::from("Clone example");
    let s2 = s1.clone();

    println!("{}", s1);
    println!("{}", s2);

    // this works because the value has been cloned -- specifically,
    // the data on the heap has been copied, making this more like a
    // "deep copy"
}

fn str_mv_ex() {
    let x = 5;
    let y = x;
    println!("int: {}", x);
    println!("int: {}", y);
    // this works as expected; integers are stored entirely on the stack, so
    // x is not invalidated by settings y = x. Variables are only invalidated
    // when there is a difference between a "shallow copy" and a "deep copy"
    // for that variable.

    // this is implemented through the Copy trait which is on types like
    // integers; if a type implements the copy trait, then a variable is
    // still valid/usable after assignment.

    // examples include int/uint, bool, float, char, and tuples that consist
    // entirely of types that implement the Copy trait.

    let s1 = String::from("String move example"); 
    let s2 = s1; // copies stack values: ptr|len|capacity, but not heap value
    // this might sound like a shallow copy, but because s1 is now invalid, it
    // is actually a "move" operation
    
    // to avoid two references to the heap data, s1 is now invalid
    //println!("{}", s1); // thus, this line will not compile; however, we could use s2
    println!("{}", s2);
}

fn str_ex() {
    // create a string literal (&str); value is known at run-time and is hard-coded into executable
    let s = "hello";

    let s = String::from(s); // :: navigates the namespace; I think this is a static method
    // this has created a String object; the stack contains ptr|len|capacity
    println!("{}", s); // println!(s) does not work; it must use a string literal to format with

    // this type of string can be mutated
    let mut s = String::from(s);
    s.push_str(", my name is Daniel");
    println!("{}", s);
} // rust calls drop automatically at the closing curly bracket