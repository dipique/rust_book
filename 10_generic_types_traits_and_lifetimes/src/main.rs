mod generics;
mod traits;
mod lifetimes;
use traits::Summary;
use traits::DefaultedSummary;

fn main() {
    generics::get_largest_ex();

    let t = traits::Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", t.summarize());
    println!("1 new tweet: {}", t.dsummarize());

    lifetimes::do_thing();
}
