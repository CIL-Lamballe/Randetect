//use std::fs::File;
//use std::io;
use std::collections::HashMap;
use crate::query::Log;

#[derive(Debug)]
enum ActivityType {
    Suspicious(i32),     // Containing nb of files manipulated.
    Misbehaving(String), // Contaning name of directory been moved.
    Normal,              // Normal user activity.
}

#[derive(Debug)]
pub struct User {
    pub username: String,
    ip: Vec<String>,
    kind: ActivityType,
}

/* Maximum suspicious action limit */
pub const BAN_LIMIT: u16 = 50;

static mut users: HashMap<String, User> = HashMap::new();

pub fn log_user(entry: Vec<Log>) {
    for relation in entry {
        users.insert(relation.get_username(), relation);
    }
    for guy in users.iter() {
        println!("Calling {:?}", guy); 
    }
}

pub mod sms {
    pub fn send() {}
}

pub mod email {

    pub fn send() -> Result<std::fs::File, std::io::Error> {
        let mut f = std::fs::File::open("email.txt")?;
        Ok(f)
    }
}
