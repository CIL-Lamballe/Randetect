use std::{thread, time};
use std::collections::HashMap;

mod alert;
mod query;

/// Loop delay in milliseconds
const TIME: u64 = 2_000;

fn main() {

    // Init table containing the misbehaving user info
    let list: HashMap<String, alert::User> = HashMap::new();

    // Deamon delay between two file audit
    let duration = time::Duration::from_millis(TIME);

//    loop {
        // Retrieve SQL relations corresponding to given user action(MOVE, DELETE, SUSPICIOUS_CWD)
        let mut query = query::select(query::Type::Move);
        query.extend(query::select(query::Type::Delete));
        query.extend(query::select(query::Type::SuspiciousCwd));

        // println!("{:?}", query);
        // let list = alert::log_user(qmove, list, query::QType::Move);
       // let list = alert::log_user(qdelete, list, query::QType::Delete);
        //let list = alert::log_user(qcwd, list, query::QType::SuspiciousCwd);

        // Accounting of action in order to determine user behavior(Normal, Suspicious, Misbehaving)
       // for user in list.iter() {
        //    println!("List= {:?}", user);
       // }

        thread::sleep(duration);
//    }
}
