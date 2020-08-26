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
    count: i32,
}

impl UserInfo {
   fn increment(&mut self) {
    self.count += 1;
   }
}

/// Accounting of action in order to determine user behavior(Normal, Suspicious, Misbehaving)
pub fn log(
    entry: Vec<Log>,
    users: &mut HashMap<String, UserInfo>
) {
    for el in entry {
        let uname = el.get_username();
        if !users.contains_key(&uname) {
            users.insert(
                uname,
                UserInfo {
                    ip: el.get_ip(),
                    count: 0,
                }
            );
        } else {
            users.get_mut(&uname).unwrap().increment();
           // println!("exist {:?}", users);
        }
    }
}
