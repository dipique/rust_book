fn main() {
    default_match_ex(1);
}

// using a "default" in a match for unspecified cases
fn default_match_ex(some_u8: u8) {
    match some_u8 {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => ()
    } // interesting that this semicolon is optional
}

// the rust equivalent of null handling
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,          // matches are exhaustive, and every case needs to
        Some(i) => Some(i + 1) // be handled; both lines are required to compile
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

// you can also create an enum that looks more like one from another language
#[repr(u8)] // optional attribute that forced a u8 underlying type
enum Color {
    White = 0,
    Blue = 2,
    Green = 5,
    Black = 9
}