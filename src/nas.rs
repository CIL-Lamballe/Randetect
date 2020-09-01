use std::process::Command;
use rusqlite::{params, Connection};
use crate::parse::UserInfo;

fn fmt_insertban(ip: &str) -> String {
    let epoch: &str = "1598967925"; // get time since epoch
    format!(
        "INSERT INTO AutoBlockIP
         VALUES (
             '{}',
             '{}',
             '0',
             '1',
             '0000:0000:0000:0000:0000:FFFF:C0A8:0170',
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
