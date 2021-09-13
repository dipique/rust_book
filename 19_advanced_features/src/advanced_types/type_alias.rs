type Kilometers = i32;
// this now acts just like an i32, but provides static type safety and
// other benefits; this is like
//      public class MyInt: int {}
// in C# but better (no boxing, no overhead)

// this can also be used for long, complex types for DRY purposes, and/or
// to more clearly indicate the intention for usage
type Thunk = Box<dyn Fn() + Send + 'static>;
// "thunk" is used for functions that will be run in the future; apparently
// thus is a common term

pub fn run() {
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    ///

    let f: Thunk = Box::new(|| println!("hi"));
}

fn takes_and_returns_long_type(t: Thunk) -> Thunk {
    panic!("not implemented")
}

// this is used for Result<T, E> where E is error. Because E is usualy std::io::Error,
// std::io has an alias:
type Result<T> = std::result::Result<T, std::io::Error>;
// That basically allows users to omit the error type, instead using Result<usize>, etc.