//use std::{thread, time};

mod query;

/* Loop dealy in milliseconds */
const TIME: u64 = 2_000;

//enum ActivityType {
//    Suspicious(i32),     // Containing nb of files manipulated.
//    Misbehaving(String), // Contaning name of directory been moved.
//    Normal,              // Normal user activity.
//}
//struct User {
//    username: String,
//    ip: Ip,
//    kind: ActivityType,
//}

fn main() {
    //let duration = time::Duration::from_millis(TIME);
    //  loop {
    query::select(query::MOVE);
    query::select(query::DELETE);
    query::select(query::SUSPICIOUS_CWD);
  //  query::select(query::SUSPICIOUS_CRWD);
    //        alert::sms::send();

    // thread::sleep(duration);
    //  }
}
