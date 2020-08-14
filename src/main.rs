use std::{thread, time};

const MAX_LOG: u16 = 2_000; // Maximum log parsed - LIFO.
const TIME: u64 = 2_000; // Loop dealy in milliseconds.

struct User {
    ip: String,
    username: String,
    filename: String,
    attempt: i32,
}

fn hello(h: u16) -> u8 {
    for i in 1..4 {
        println!("Hello, world! {}", h - i);
    }
    5
}