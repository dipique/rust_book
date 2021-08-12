// the primary purpose of lifetimes is to prevent dangling references

use super::traits; // hey, that's how you use a module in the same directory

// &i32        // reference
// &'a i32     // reference with explicit lifetime
// &'a mut i32 // mutable reference with explicit lifetime


// this requires that the returned reference will survive
// as long as the shortest parameter reference lifetime;
// lifetime parameters never modify lifetimes, only
// establish them 
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// lifetimes are required for both x and y because
// either reference could be returned; a third
// parameter would not necessarily need to have an
// explicit lifetime unless its reference could
// potentially be returned as well

fn first<'a>(x: &'a str, y: &str) -> &'a str {
    println!("You know what isn't first? {}", y);
    x
}

// when returning a reference from a function, the
// lifetime parameter of the return reference must
// match at least one of the parameters; thus the
// following would not compile:

// fn long_string<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str() // returns a reference to a string
//     // that will immediately go out of scope at the end
//     // of the function

//     // lifetime parameters do not change the lifetime,
//     // it just annotates it

//     // I am not 100% clear on why lifetimes are not
//     // implicit, but presumably that will become more
//     // clear over time.
//
//     // Aha! See the 3 rules below. Each reference has
//     // a lifetime parameter (even if not written), and
//     // the output must match 1. It is ambiguous which
//     // it should use. I assume this methodology is used
//     // because anything more sophisticated would have
//     // crazy impact on performance.
// }

// lifetimes go in the function signature, not the
// body; within the function, rust doesn't need help

pub fn do_thing() {
    let string1 = String::from("ABCD");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let s1 = "s1";
    let mut l: &str;
    {
        let s2 = "s2".to_string();
        l = longest(s1, s2.as_str());
        println!("Longest now is: {}", l);
    }

    // Cannot compile with next line because
    // l would have a longer lifetime than the
    // shorter of the 2 parameters to longest()

    // println!("And now it is: {}", l);

    // lifetimes are required for how the borrow
    // checker works; they disambiguate references
}

struct ImportantExcerpt<'a> {
    part: &'a str
}

// this annotation means that an instance of ImportantExcerpt
// cannot outlive the reference held in the "part" field; the
// code won't compile without this annotation

pub fn imp_excerpt() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence
    };
}

// lifetime annotations are required when the lifetime cannot
// be reasonably inferred by the compiler. Some annotations
// can be inferred, such as first_word from chapter 4

// fn first_word(s: &str) -> &str { ... }

// inferred use lifetime elision rules:

// fn first_word<'a>(s: &'a str) -> &a' str { ... }

// There are 3 lifetime rules the compiler follows

// 1) Each parameter that is a reference gets its own lifetime parameter
// 2) If there is exactly one input lifetime parameter, that lifetime is
//    assigned to all output lifetime parameters
// 3) If there are multiple input lifetime parameters, but one of them is
//    &self or &mut self because this is a method, the lifetime of self is
//    assigned to all output lifetime parameters

// lifetime parameter is still required even if members don't use it, if it
// is present on the struct
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        // lifetime of &self is assigned to return value
        println!("Attention please: {}", announcement);
        self.part
    }
}

// static lifetimes last for the duration of the program
// let s: &'static str = "I have a static lifetime";

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() { x } else { y }
}

// just... wow. What an exhausting section. How did this not
// deserve its own chapter?