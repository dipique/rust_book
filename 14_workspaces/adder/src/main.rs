use add_one;

fn main() {
    let num = 10;
    println!(
        "Hello, world! {} plus one is {}!",
        num,
        add_one::add_one(num)
    );
}

// to run this specific package from the workspace directory:
//    cargo run -p adder

// tests can be run for a specific package using the same format