fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

pub fn run() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);

    assert_eq!(fn_as_closure(), fn_as_named_fn());

    enum_init_pattern();

    let my_fn = returns_closure();
    println!("Using my_fn: {}", my_fn(4));
}

fn fn_as_closure() {
    let list_of_nums = vec![1,2,3];
    let list_of_strs: Vec<String> =
        list_of_nums.iter().map(|i| i.to_string()).collect();
}

fn fn_as_named_fn() {
    let list_of_nums = vec![1,2,3];
    let list_of_strs: Vec<String> =
        list_of_nums.iter().map(ToString::to_string).collect();
}

// you can use an enum to create initializers/constructors
// without requiring a closure

enum Status {
    Value(u32),
    Stop,
}

fn enum_init_pattern() {
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}

// returning a closure; note that closures are unsized, which means that it will
// need to be behind a pointer
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}