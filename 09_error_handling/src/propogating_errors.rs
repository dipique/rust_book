use std::fs:File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// same thing using ? operator
fn with_prop_operator() -> Result<String, io::Error {
    let mut f = File::open("hello.txt")?; // return error if it happens
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn more_concise_with_chaining() -> Result<String, io::Error {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s);
}

fn most_concise() -> Result<String, io::Error {
    fs::read_to_string("hello.txt") //turns out there's already a function to do this
}

// note that the ? operator has to be used in a function that returns Result or Option
// or another operator that implements the Try trait