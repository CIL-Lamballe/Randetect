mod alert;
mod parse;
mod query;

use alert::{email, sms};
use parse::Behavior;
use query::Type;
use rusqlite::Connection;
use std::{collections::HashMap, env, thread, time::Duration};

//const DB: &str = "/var/log/synolog/.SMBXFERDB";
const DB: &str = "/home/antoine/RanDetect/.SMBXFERDB";

pub struct Cdtl {
    user: String,
    pwd: String,
    sys: String,
    folder: String,
    smsusr: String,
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

    let conn = match Connection::open(DB) {
        Err(conn) => panic!("Could not reach/open database {}", DB),
        Ok(conn) => conn,
    };
    let mut id = query::updated_id(&conn) - 2_500;
   // println!("Id:{}", id);

    let mut list: HashMap<String, parse::UserInfo> = HashMap::new();

    //    loop {
    let mut query = query::select(&conn, Type::Move, &id);
    query.extend(query::select(&conn, Type::Delete, &id));
    query.extend(query::select(&conn, Type::SuspiciousCwd, &id));

    id = query::updated_id(&conn);

    parse::log(query, &mut list);
    for user in list.iter() {
        let (name, info) = user;
        for beh in info.get_behaviors() {
            match beh {
                Behavior::Delete(c) if *c >= 50 => {
                    // println!("BAN of {} because he/she as been deleting {} files", name, *c);
                    email::send(&name, info, "Move");
                }
                Behavior::Suspicious(c) if *c >= 50 => {
                    //  println!("BAN of {} for having suspicious activity", name);
                    sms::send(&var, &name, info);
                }
                Behavior::Move(s) => {
                    //  println!("{} moved the folder {}", name, *s);
                    email::send(&name, info, "Move");
                }
                _ => (),
            }
        }
        // println!("{:?}",user.UserInfo);
    }
    // thread::sleep(duration);
    //    }
}
