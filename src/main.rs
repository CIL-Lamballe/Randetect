//use std::{thread, time};
use std::collections::HashMap;

mod alert;
mod query;

/* Loop dealy in milliseconds */
const TIME: u64 = 2_000;

fn main() {
    let mut list: HashMap<String, alert::User> = HashMap::new();
    //let duration = time::Duration::from_millis(TIME);
    //  loop {
    let qmove = query::select(query::MOVE);
    let qdelete = query::select(query::DELETE);
    let qcwd = query::select(query::SUSPICIOUS_CWD);

    alert::log_user(qmove, list);

    //  query::select(query::SUSPICIOUS_CRWD);
    //        alert::sms::send();

    // thread::sleep(duration);
    //  }
}
