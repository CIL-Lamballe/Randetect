use std::process::Command;
use rusqlite::{params, Connection};
use crate::parse::UserInfo;

use std::time::SystemTime;

fn fmt_insertban(ip: &str) -> String {
    let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let epoch = time.as_secs().to_string(); // get time since epoch
    format!(
        "INSERT INTO AutoBlockIP
         VALUES (
             '{}',
             '{}',
             '0',
             '1',
             '0000:0000:0000:0000:0000:0000:0000:0000',
             '0',
             '0'
             );", ip, epoch
        )
}

pub fn ban(conn: &Connection, info: &UserInfo) {
    for ip in info.get_ips().iter() {
        let insertstmt = fmt_insertban(&ip);
        println!("{}", &insertstmt);
        conn.execute(&insertstmt, params![],).unwrap();
    }
}

pub fn poweroff() {
    let output = Command::new("bash")
        .arg("-c")
        .arg("shutdown -h now")
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    // Debug should send sms failed to poweroff and send when powering off
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}
