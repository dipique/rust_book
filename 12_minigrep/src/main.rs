use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let cfg = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Failed with error: {}", err);
        process::exit(1);
    });

    if let Err(err) = minigrep::run(cfg) {
        println!("Application error: {}", err);
        process::exit(1);
    }
}