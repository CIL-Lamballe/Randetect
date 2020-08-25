//use std::fs::File;
//use std::io;
use crate::query::Log;
use std::collections::HashMap;

#[derive(Debug)]
pub enum ActivityType {
    Suspicious(i32),     // Containing nb of files manipulated.
    Misbehaving(String), // Contaning name of directory been moved.
    Normal,              // Normal user activity.
}

#[derive(Debug)]
pub struct User {
    ip: Vec<String>,
    kind: ActivityType,
    //  kind: Vec<ActivityType>,
}

/* Maximum suspicious action limit */
pub const BAN_LIMIT: u16 = 50;

//pub fn log_user(
//    entry: Vec<Log>,
//    mut users: HashMap<String, User>,
//    query: QType,
//) -> HashMap<String, User> {
//    for relation in entry {
//        users.insert(
//            relation.get_username(),
//            User {
//                ip: {
//                    let u = users.get(&relation.get_username());
//                    println!("ip {:?}", u);
//                    // println!("pi {:?}", relation);
//                    // println!("ip {:?}", ip);
//                   // match u {
//                    //    Some(u) => v.push(relation.get_ip()), //println!("ip {:?}", u),
//                     //   None => {
//                      //      let v = Vec::new();
//                       //     v.push(relation.get_ip());
//                        //    v
//                       // }
//                    }
//                    //     Vec::new()
//                },
//                kind: {
//                    match query {
//                        QType::Move => ActivityType::Misbehaving(String::from("DIRNAME")),
//                        QType::Delete | QType::SuspiciousCwd => ActivityType::Suspicious(42),
//                        _ => ActivityType::Normal,
//                    }
//                    // if !map.get(&relation.kind) let mut k = Vec::new();
//                    // v.push()
//                },
//            },
//            );
//    }
//    users
//}

pub mod sms {
    pub fn send() {}
}

pub mod email {

    pub fn send() -> Result<std::fs::File, std::io::Error> {
        let f = std::fs::File::open("email.txt")?;
        Ok(f)
    }
}
