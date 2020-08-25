//use std::fs::File;
//use std::io;
use crate::query::Log;
use std::collections::HashMap;

/// Maximum of suspicious actions
const BAN_LIMIT: u16 = 50;

#[derive(Debug)]
enum Behavior {
    Suspicious(i32),     // Containing nb of files manipulated.
    Misbehaving(String), // Contaning name of directory been moved.
    Normal,              // Normal user activity.
}

#[derive(Debug)]
pub struct UserInfo {
    ip: String,
//    ip: Vec<String>,
//    kind: ActivityType,
    //  kind: Vec<ActivityType>,
}

/// Accounting of action in order to determine user behavior(Normal, Suspicious, Misbehaving)
pub fn log(
    entry: Vec<Log>,
    mut users: HashMap<String, UserInfo>
) -> HashMap<String, UserInfo> {

    for relation in entry {
        users.insert(
            relation.get_username(),
            UserInfo {
                ip: relation.get_ip(),
            },
        );
    }
    users
}
