fn main() {
    concise_alt(Some(6));
    concise_alt(Some(3));
    concise_alt(None);
}

// handles all cases, but unnecessarily verbose
// ONLY in void function (if there was a return,
// we need this so we handle exhaustively)
fn match_anti_pattern(some_u8: Option<u8>) {
    match some_u8 {
        Some(3) => println!("three"),
        _ => ()
    }
}

fn concise_alt(some_u8: Option<u8>) {
    // concise, but non-exhaustive
    if let Some(3) = some_u8 { 
        println!("three")
    }

    // this is a shorthand single match, but in this
    // case a standard conditional works just as well
    if Some(3) == some_u8 { 
        println!("three")
    }

    // but "if let" allows us to use pattern matching,
    // such as this conditional that checks for whether
    // a value exists
    if let Some(u8_val) = some_u8 {
        println!("A value was provided: {}", u8_val)
    } else {
        println!("No value was provided");
    }
}