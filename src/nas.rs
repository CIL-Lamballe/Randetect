use crate::parse::UserInfo;
use std::{thread, process::Command, time::Duration};


pub fn cmd_exec(cmd: &str) -> (String, String, String) {
    println!("{}", cmd);
    let output = Command::new("bash")
        .arg("-c")
        .arg(cmd)
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));


    // Debug
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    (format!("{}", output.status),
        format!("{}", String::from_utf8_lossy(&output.stdout)),
            format!("{}", String::from_utf8_lossy(&output.stderr)))
}

fn ban_profile(ip: &str) -> String {
    "synowebapi --exec".to_string()
        + " profile=\\{\\\"global\\\":\\{\\\"policy\\\":\\\"none\\\",\\\"rules\\\":\\[\\{\\\"enable\\\":true,\\\"name\\\":\\\"\\\",\\\"port_direction\\\":\\\"\\\",\\\"port_group\\\":\\\"all\\\",\\\"ports\\\":\\\"all\\\",\\\"protocol\\\":\\\"all\\\",\\\"source_ip_group\\\":\\\"ip\\\",\\\"source_ip\\\":\\\""
        + ip
        + "\\\",\\\"policy\\\":\\\"drop\\\",\\\"log\\\":false\\}\\]\\},\\\"name\\\":\\\"custom\\\"\\}"
        + " profile_applying=true api=SYNO.Core.Security.Firewall.Profile method=set version=1"
}

fn apply_profile() -> String {
    "synowebapi --exec name=\"custom\" profile_applying=true api=SYNO.Core.Security.Firewall.Profile.Apply method=start version=1".to_string()
}

pub fn ban(info: &UserInfo) {
    for ip in info.get_ips().iter() {
        let cmd = ban_profile(&ip);
        cmd_exec(&cmd);
        thread::sleep(Duration::from_millis(500));
        let cmd = apply_profile();
        let (status, stdout, stderr) = cmd_exec(&cmd);
        thread::sleep(Duration::from_millis(500));
    }
}

pub fn poweroff() {
    cmd_exec("shutdown -h now");
}
