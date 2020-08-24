//use std::{thread, time};

mod alert;
mod query;

/* Loop dealy in milliseconds */
const TIME: u64 = 2_000;

fn main() {
    //let duration = time::Duration::from_millis(TIME);
    //  loop {
    let qmove = query::select(query::MOVE);
    let qdelete = query::select(query::DELETE);
    let qcwd = query::select(query::SUSPICIOUS_CWD);

    alert::log_user(qmove);

    //  query::select(query::SUSPICIOUS_CRWD);
    //        alert::sms::send();

    // thread::sleep(duration);
    //  }
}
