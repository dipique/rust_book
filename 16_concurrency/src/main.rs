use std::thread;
use std::time::Duration;

mod message_passing;
mod shared_state_concurrency;
mod extensible_concurrency;

fn main() {
    use_join_handles();
    move_closures();
    message_passing::run();
    shared_state_concurrency::run();
    extensible_concurrency::run();
}

fn use_join_handles() {
    // where in C# we could use a task to await, rust has a joinhandle
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the spawned thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // wait for threads to complete
    handle.join().unwrap(); // blocks current thread until this thread terminates
}

fn basic_threads() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
            //forces thread to stop execution, allowing another
            // thread to run
        }
    });

    for i in 1..5 {
        println!("hi number {} from the spawned thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // when the main thread closes, all related threads will also
    // close even if they haven't completed
}

fn move_closures() {
    let v = vec![1,2,3];

    let handle = thread::spawn(move || { // force closure to capture (take ownership of) values
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap(); // this also returns a result, which is the result of the closure--again, like task/promise
}