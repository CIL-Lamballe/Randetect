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

    fn update(&mut self, newip: String, t: Type, dir: String) {
        match t {
            Type::Delete => {
                for each in &mut self.kind {
                    match each {
                        Behavior::Delete(c) => *c = *c + 1,
                        _ => (),
                    };
                }
            }
            Type::SuspiciousCwd => {
                for each in &mut self.kind {
                    match each {
                        Behavior::Suspicious(c) => *c = *c + 1,
                        _ => (),
                    };
                }
            }
            Type::Move => self.kind.push(Behavior::Move(dir)),
        }
        if !self.ip.contains(&newip) {
            self.ip.push(newip);
        }
    }

    pub fn get_behaviors(&self) -> &Vec<Behavior> {
        &self.kind
    }

    pub fn get_ips(&self) -> &Vec<String> {
        &self.ip
    }
}

/// Accounting of action in order to determine user behavior(Normal, Suspicious, Misbehaving)
pub fn log(entry: Vec<Log>, users: &mut HashMap<String, UserInfo>) {
    for el in entry {
        let uname = el.get_username();
        if users.contains_key(&uname) {
            users
                .get_mut(&uname)
                .unwrap()
                .update(el.get_ip(), el.get_kind(), el.get_dir());
        } else {
            users.insert(
                uname,
                UserInfo::new(el.get_ip(), el.get_kind(), el.get_dir()),
            );
        }
    }
}
