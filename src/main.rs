//use std::{thread, time};
use std::collections::HashMap;

mod alert;
mod query;

/* Loop dealy in milliseconds */
const TIME: u64 = 2_000;

fn main() {
    let list: HashMap<String, alert::User> = HashMap::new();
    //let duration = time::Duration::from_millis(TIME);

    // loop {
    let qmove = query::select(query::MOVE);
    let qdelete = query::select(query::DELETE);
    //    let qcrwd = query::select(query::SUSPICIOUS_CRWD);
    let qcwd = query::select(query::SUSPICIOUS_CWD);

    let list = alert::log_user(qmove, list, query::QType::Move);
    let list = alert::log_user(qdelete, list, query::QType::Delete);
 //   let list = alert::log_user(qcwd, list, query::QType::SuspiciousCwd);

    for user in list.iter() {
        println!("List= {:?}", user);
    }
    //        alert::sms::send();

    // thread::sleep(duration);
    //  }
}
