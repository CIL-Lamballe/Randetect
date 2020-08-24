//use std::fs::File;
//use std::io;
use crate::query::Log;
use std::collections::HashMap;

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

pub fn log_user(entry: Vec<Log>, mut users: HashMap<String, User>) -> HashMap<String, User> {
    for relation in entry {
        users.insert(
            relation.get_username(),
            User {
                username: relation.get_username(),
                ip: {
                    let mut v = Vec::new();
                    v.push(relation.get_ip());
                    v
                },
                kind: ActivityType::Normal,
            },
        );
    }
    users
}

pub mod sms {
    pub fn send() {}
}

pub mod email {

    pub fn send() -> Result<std::fs::File, std::io::Error> {
        let f = std::fs::File::open("email.txt")?;
        Ok(f)
    }
}
