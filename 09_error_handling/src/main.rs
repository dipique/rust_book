// to enable backtrace in powershell: $Env:RUST_BACKTRACE=1

// panics are unrecoverable

// they should be used when the code can enter a bad state with invalid
// or contradictory values

// also note that the main function returns () by default and is restricted in its return
// types, but one valid return type is Result, e.g.
use std::error::Error;
use std::fs::File;
fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;
    Ok(())
} // for these pirposes, Box<dyn Error>> means "any error"

fn trigger_panic() {
    panic!("Crash and burn");
}

fn panic_in_std_lib() {
    let v = vec![1, 2, 3];
    v[99];
}

fn handle_errors() {
    let f = File::open("Hello.txt");
    // to check type if intellisense isn't helping, try declare with the unit type:
    // let f: () = File.open("Hello.txt"); // the compiler will show the expected type
    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}

use std::io::ErrorKind;

fn match_error_types() {
    let f = File::open("Hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(ec) => panic!("Error creating the file: {:?}", ec),
            },
            other_error => { // other_error is a variable that contains the value of the defaulted err
                panic!("Problem opening the file: {:?}", other_error)
            }
        }
    };
}

fn match_error_types_concise_alt() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn unwrap_calls_panic_automatically_on_error() {
    let f = File::open("hello.txt").unwrap();
    // if file doesn't exist, panic will occur automatically
}

fn using_expect() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
    // unwrap that panics with a provided message
}

// functions like unwrap() and expect() are considered
// placeholders for more robust error handling, except
// when you have more information than the compiler
use std::net::IpAddr;
fn information_assymetry() {
    // we can safely unwrap because we can guarantee no panic
    let home: IpAddr = "127.0.0.1".parse().unwrap();
}