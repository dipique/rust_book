// rust has channels to communicate between threads; each has two halves, a transmitter and a receiver
// this means that channels are one-way. A channel is "closed" if either half is dropped.

use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::sync::mpsc::Sender;

pub fn run() {
    simple();
    mult_values();
    multiple_producers();
}

fn simple() {
    let (tx, rx) = mpsc::channel(); // multiple producer, single consumer

    thread::spawn(move || {
        let val = String::from("hi");
        thread::sleep(Duration::from_millis(100));
        tx.send(val).unwrap();
    });

    println!("Waiting for message...");
    let received = rx.recv().unwrap(); // block thread and wait until value is sent
    println!("Got: {}", received);
}

fn sender(tx: Sender<String>, vals: Vec<String>, ms_wait: u64) {
    for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_millis(ms_wait));
    }
}

fn mult_values() {
    println!("*** mult_values ***");
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || { sender(tx, vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
    ], 250); });

    // this will wait for messages until the transmitter is destroyed
    for received in rx {
        println!("Got: {}", received);
    }
}

fn multiple_producers() {
    println!("*** multiple_producers ***");
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || { sender(tx1, vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
    ], 50); });

    thread::spawn(move || { sender(tx, vec![
        String::from("more"),
        String::from("messages"),
        String::from("from"),
        String::from("you"),
    ], 180); });

    for received in rx {
        println!("Got: {}", received);
    }
}