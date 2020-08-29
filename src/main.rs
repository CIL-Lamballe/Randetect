mod alert;
mod parse;
mod query;

use std::{collections::HashMap, env, thread, time::Duration};
use rusqlite::Connection;
use parse::Behavior;
use alert::{email, sms};
use query::Type;

const DB: &str = "/home/antoine/RanDetect/.SMBXFERDB";

pub struct Cdtl {
    user: String,
    pwd: String,
    sys: String,
    folder: String,
    smsusr: String,
}

impl Cdtl {
    pub fn get_user(&self) -> &str {
        &self.user
    }

    pub fn get_pwd(&self) -> &str {
        &self.pwd
    }

    pub fn get_sys(&self) -> &str {
        &self.sys
    }

    pub fn get_folder(&self) -> &str {
        &self.folder
    }

    pub fn get_smsusr(&self) -> &str {
        &self.smsusr
    }
}

/// Loop delay in milliseconds
const TIME: u64 = 2_000;

/// Get environment variable for lftp use
fn getenv(var: &str) -> String {
    match env::var(var) {
        Ok(val) => val,
        Err(e) => panic!("{} : {}", var, e),
    }
}

fn env_variables() -> Cdtl {
    let crdtl = getenv("CRDTL");
    Cdtl {
        user: crdtl[..10].to_string(),
        pwd: crdtl[10..18].to_string(),
        sys: getenv("TARGETSYS"),
        folder: getenv("FOLDER"),
        smsusr: getenv("SMSUSR"),
    }
}

fn main() {
    let var: Cdtl = env_variables();

    let duration = Duration::from_millis(TIME);

    let conn = Connection::open(DB).unwrap();

    let mut list: HashMap<String, parse::UserInfo> = HashMap::new();

    //    loop {
    let mut query = query::select(&conn, Type::Move);
    query.extend(query::select(&conn,Type::Delete));
    query.extend(query::select(&conn,Type::SuspiciousCwd));

    parse::log(query, &mut list);
    for user in list.iter() {
        let (name, info) = user;
        for beh in info.get_behaviors() {
            match beh {
                Behavior::Delete(c) if *c >= 50 => {
                   // println!("BAN of {} because he/she as been deleting {} files", name, *c);
                    email::send(&name, info, "Move");
                },
                Behavior::Suspicious(c) if *c >= 50 => {
                  //  println!("BAN of {} for having suspicious activity", name);
                    sms::send(&var, &name, info);
                },
                Behavior::Move(s) => {
                  //  println!("{} moved the folder {}", name, *s);
                    email::send(&name, info, "Move");
                },
                _ => (),
            }
        }
        // println!("{:?}",user.UserInfo);
    }
    // thread::sleep(duration);
    //    }
}
