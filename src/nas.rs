use crate::parse::UserInfo;
use std::process::Command;

fn set_ban_profile(ip: &str) -> String {
    "synowebapi --exec".to_string()
    + " profile=\\{\\\"global\\\":\\{\\\"policy\\\":\\\"none\\\",\\\"rules\\\":\\[\\{\\\"enable\\\":true,\\\"name\\\":\\\"\\\",\\\"port_direction\\\":\\\"\\\",\\\"port_group\\\":\\\"all\\\",\\\"ports\\\":\\\"all\\\",\\\"protocol\\\":\\\"all\\\",\\\"source_ip_group\\\":\\\"ip\\\",\\\"source_ip\\\":\\\""
    + ip
    + "\\\",\\\"policy\\\":\\\"drop\\\",\\\"log\\\":false\\}\\]\\},\\\"name\\\":\\\"custom\\\"\\}"
    + " profile_applying=true api=SYNO.Core.Security.Firewall.Profile method=set version=1"
}

pub fn ban(info: &UserInfo) {
    for ip in info.get_ips().iter() {
        let cmd = set_ban_profile(&ip);
        println!("{}", cmd);
        //        let output = Command::new("bash")
        //            .arg("-c")
        //            .arg("shutdown -h now")
        //            .output()
        //            .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

        // Debug should send sms failed to poweroff and send when powering off
        //        println!("status: {}", output.status);
        //        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        //        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
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
