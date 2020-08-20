use rusqlite::{Connection, Result, NO_PARAMS};
use std::{thread, time};

mod alert;
mod config;
mod database;

//enum ActivityType {
//    Suspicious(i32),     // Containing nb of files manipulated.
//    Misbehaving(String), // Contaning name of directory been moved.
//    Normal,              // Normal user activity.
//}
//
//enum Ip {
//    V4(String),
//    V6(String),
//}
//
//struct User {
//    username: String,
//    ip: Ip,
//    kind: ActivityType,
//}

fn main() {
    let db = format!("{}{}", config::DBPATH, config::DBNAME);

    let duration = time::Duration::from_millis(config::TIME);

    loop {
        println!("Loop");
        database::huge_delete(&db);
        thread::sleep(duration);
        //        database::dir_move();
        //        alert::sms::send();
    }
}
