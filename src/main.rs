use std::collections::HashMap;
use std::{thread, time, env};

mod alert;
mod parse;
mod query;

pub struct Cdtl {
    user: String,
    pwd: String,
    sys: String,
    folder: String,
    smsusr: String,
}

impl Cdtl {
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
        user: crdtl[..9].to_string(),
        pwd: crdtl[10..17].to_string(),
        sys: getenv("TARGETSYS"),
        folder: getenv("FOLDER"),
        smsusr: getenv("SMSUSR"),
    }
}

fn main() {

    let var: Cdtl = env_variables();

    alert::sms::send(&var);

/*
    let duration = time::Duration::from_millis(TIME);

    let mut list: HashMap<String, parse::UserInfo> = HashMap::new();

    //    loop {
    let mut query = query::select(query::Type::Move);
    query.extend(query::select(query::Type::Delete));
    query.extend(query::select(query::Type::SuspiciousCwd));

    parse::log(query, &mut list);
    for user in list.iter() {
        let (name, info) = user;
        for beh in info.get_behaviors() {
            match beh {
                parse::Behavior::Delete(c) if *c >= 50 => println!(
                    "BAN of {} because he/she as been deleting {} files",
                    name, *c
                ),
                parse::Behavior::Suspicious(c) if *c >= 50 => {
                    println!("BAN of {} for having suspicious activity", name);
                    alert::sms::send(&name, info, "Suspicious")
                }
                parse::Behavior::Move(s) => {
                    println!("{} moved the folder {}", name, *s);
                    alert::email::send(&name, info, "Move")
                }
                _ => (),
            }
        }
        // println!("{:?}",user.UserInfo);
    }
    // thread::sleep(duration);
    //    }
*/
}
