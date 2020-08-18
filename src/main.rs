use std::{thread, time};

/* Maximum log parsed - LIFO. */
const MAX_LOG: u16 = 2_000;

/* Loop dealy in milliseconds. */
const TIME: u64 = 2_000;

/* Path to Synology logs. */
const DBPATH: &str = "/var/log/synolog/";

/* Database containing file logs. */
const DB: &str = ".SMBXFERDB";

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
