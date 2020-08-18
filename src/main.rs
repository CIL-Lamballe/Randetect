use std::{thread, time};

const MAX_LOG: u16 = 2_000; // Maximum log parsed - LIFO.
const TIME: u64 = 2_000; // Loop dealy in milliseconds.
const DBPATH: &str = "/var/log/synolog/"; // Path to Synology logs.
const DB: &str = ".SMBXFERDB"; // Database containing file logs.

enum ActivityType {
    Suspicious(i32),     // Containing nb of files manipulated.
    Misbehaving(String), // Contaning name of directory been moved.
    Normal,              // Normal user activity.
}

struct User {
    ip: String,
    username: String,
    kind: ActivityType,
}

fn main() {
    let duration = time::Duration::from_millis(TIME);
    loop {
        println!("Hello, world!");
        thread::sleep(duration);
    }
}
