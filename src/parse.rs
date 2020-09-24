//use std::fs::File;
//use std::io;
use crate::query::{Log, Type};
use std::collections::HashMap;

#[derive(Debug)]
pub enum Behavior {
    Delete(i32),     // Containing nb of files deleted.
    Suspicious(i32), // Containing nb of files manipulated.
    Move(String),    // Contaning name of directory been moved.
}

#[derive(Debug)]
pub struct UserInfo {
    ip: Vec<String>,
    kind: Vec<Behavior>,
}

impl UserInfo {
    fn new(ip: String, t: Type, dir: String) -> UserInfo {
        UserInfo {
            ip: {
                let mut n = Vec::new();
                n.push(ip);
                n
            },
            kind: {
                let mut k = Vec::new();
                k.push(match t {
                    Type::Delete => Behavior::Delete(1),
                    Type::SuspiciousCwd => Behavior::Suspicious(1),
                    Type::Move => Behavior::Move(dir),
                });
                k
            },
        }
    }

    pub fn get_behaviors(&self) -> &Vec<Behavior> {
        &self.kind
    }

    pub fn get_ips(&self) -> &Vec<String> {
        &self.ip
    }
}

#[warn(clippy::map_entry)]
/// Accounting of action in order to determine user behavior(Normal, Suspicious, Misbehaving)
pub fn log(entry: Vec<Log>, users: &mut HashMap<String, UserInfo>) {
    for el in entry {
        let uname = el.get_username();
        let update = users
            .entry(uname)
            .or_insert_with(|| UserInfo::new(el.get_ip(), el.get_kind(), el.get_dir()));
        *update = UserInfo::new(el.get_ip(), el.get_kind(), el.get_dir());
    }
}
