mod alert;
mod parse;
mod query;
mod nas;

use alert::{email, sms};
use parse::Behavior;
use query::Type;
use rusqlite::Connection;
use std::{collections::HashMap, env, thread, time::Duration};

/// Path to file log db
//const DB: &str = "/var/log/synolog/.SMBXFERDB";
const DB: &str = "/home/antoine/RanDetect/.SMBXFERDB";

/// Maximum of suspicious actions
const BAN_LIMIT: i32 = 50;

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
        Err(conn) => panic!("Could not reach/open database {} {}", DB, conn),
        Ok(conn) => conn,
    };
    //let mut id = query::updated_id(&conn);// - 2_500;
    let mut id = query::updated_id(&conn) - 2_500;
    loop {
        let mut list: HashMap<String, parse::UserInfo> = HashMap::new();

        let mut query = query::select(&conn, Type::Move, &id);
        query.extend(query::select(&conn, Type::Delete, &id));
        query.extend(query::select(&conn, Type::SuspiciousCwd, &id));

        id = query::updated_id(&conn);

        parse::log(query, &mut list);
        for user in list.iter() {
            let (name, info) = user;
            for beh in info.get_behaviors() {
                match beh {
                    Behavior::Delete(c) if *c >= BAN_LIMIT => {
                        nas::ban(&name, info, *c);
                        email::send(&name, info, "Delete");
                    }
                    Behavior::Suspicious(c) if *c >= BAN_LIMIT => {
                        nas::ban(&name, info, *c);
                        //sms::send(&var, &name, info);
                    }
                    Behavior::Move(s) => {
                        email::send(&name, info, "Move");
                    }
                    _ => (),
                }
            }
        }
        thread::sleep(duration);
    }
}
