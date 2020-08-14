use std::{thread, time};

const MAX_LOG: u16 = 2_000; // Maximum log parsed - LIFO.
const TIME: u64 = 2_000; // Loop dealy in milliseconds.

struct User {
    ip: String,
    username: String,
    filename: String,
    attempt: i32,
}

fn main() {
    let duration = time::Duration::from_millis(2000);
    loop {
        println!("Hello, world!");
        thread::sleep(duration);
    }
}