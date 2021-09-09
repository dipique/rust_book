use std::sync::{Mutex, Arc};
use std::thread;

pub fn run() {
    basic_mutex();
    threaded_mutex();
}

fn basic_mutex() {
    let m = Mutex::new(5);
    // note that this provide interior mutability; the mutex
    // is immutable, but can provide a mutable reference to 
    // the contents

    {
        let mut num = m.lock().unwrap();
        *num = 6;
        
        // at the end of the scope, the lock is released
    }

    println!("m = {:?}", m);
}

fn threaded_mutex() {
    // using an atomic reference count (like rc, but thread-safe) to handle passing mutex between threads
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}