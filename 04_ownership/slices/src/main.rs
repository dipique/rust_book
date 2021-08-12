fn main() {
    let s = String::from("Test string");
    println!("Index of end of first word in string '{}' is: {}", s, first_word(&s));

    let hw = String::from("Hello world");
    let len = hw.len();
    let hello = &s[..5]; // string slice "Hello" // same as [0..5]
    let world = &s[6..]; // string slice "World" // same as [6..11] or [6..len]
    let full  = &s[..];  // full string          // same as [0..11] or [0..len]
    // slice indices are non-inclusive for the ending index

    let fw = &s[..first_word(&s)];
    println!("First word in string '{}' is: {}", s, fw); // heyyy it worked!

    // slices have to be at valid UTF-8 character boundaries or there will be an error;
    // does that mean looping through bytes to use with slices can be a bad idea? Fine
    // with ascii text at least.

    let s2 = String::from("Second test string");
    println!("First word in string '{}' is: {}", s2, first_word_slice(&s2));

    let s3 = String::from("AllOneWord_TestString");
    println!("First word in string '{}' is: {}", s3, first_word_slice(&s3));
}

// non-slice version that only gets the index of the end of the first word
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    // iter() returns each element in a collection
    // enumerate() returns each element as part of a tuple that includes the index
    // we have &item because it is a reference to an element in the array
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// this version will ensure that the reference to the string stays valid
fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    s
}