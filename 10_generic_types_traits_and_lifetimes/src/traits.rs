pub trait Summary {
    fn summarize(&self) -> String;
}

// traits look like interfaces!

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// create a default implementatino
pub trait DefaultedSummary {
    fn summarize_author(&self) -> String;
    fn dsummarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl DefaultedSummary for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author.to_string()
    }
    fn dsummarize(&self) -> String {
        self.summarize()
    }
}

impl DefaultedSummary for Tweet { 
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
} // use default implementation

// require parameter to have a trait
pub fn notify(item: &impl DefaultedSummary) {
    println!("Breaking news! {}", item.dsummarize());
}

// notify is syntactical sugar for a longer form called
// "trait bound":
pub fn dnotify<T: DefaultedSummary>(item: &T) {
    println!("Breaking news! {}", item.dsummarize());
}

// this also allows us to create multiple parameters that
// we force to be the same type
pub fn mnotify<T: DefaultedSummary>(first: T, second: T) {
    println!("{} {}", first.summarize_author(), second.summarize_author())
}

// with multiple traits:
// fn some_function<T: Display + Clone, U: Display + Clone>(t: &T, u: &u) -> i32 {}

// we can also use a where clause to express the same restrictions:
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Summary + Clone,
          U: Summary + Clone
{ 1 }

// to return a type with a trait
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: "".to_string(),
        content: String::from(""),
        reply: true,
        retweet: false,
    }
}
// however, you can only return one type from a function, even if
// the different types implement the same trait :(


//// conditionally applied traits
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self { // hey! cool!
        Self { x, y }
    }
}

// this function is only available if T implements these traits
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// we can also implement a function for any type that implements a trait
// impl<T: Display> ToString for T { }
