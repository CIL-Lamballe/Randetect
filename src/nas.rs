use crate::parse::UserInfo;
use serde_json::Value;
use std::{process::Command, thread, time::Duration};

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

    (
        format!("{}", output.status),
        format!("{}", String::from_utf8_lossy(&output.stdout)),
        format!("{}", String::from_utf8_lossy(&output.stderr)),
    )
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

fn update_profile(task_id: &str) -> String {
    "synowebapi --exec task_id=".to_string()
        + task_id
        + " api=SYNO.Core.Security.Firewall.Profile.Apply method=status version=1"
}

fn close_request() -> String {
    "synowebapi --exec api=SYNO.Core.Security.Firewall.Profile.Apply method=stop version=1"
        .to_string()
}

pub fn ban(info: &UserInfo) {
    for ip in info.get_ips().iter() {
        // Format ban request
        let cmd = ban_profile(&ip);
        cmd_exec(&cmd);
        thread::sleep(Duration::from_millis(800));

        // Apply new profile and request task_id
        let cmd = apply_profile();
        let (_status, stdout, _stderr) = cmd_exec(&cmd);
        thread::sleep(Duration::from_millis(800));
        let v: Value = serde_json::from_str(&stdout).unwrap();
        let v: Value = serde_json::from_str(&v["data"].to_string()).unwrap();
        let v = v["task_id"].to_string();

        // Update the profile using task_id
        let cmd = update_profile(&v);
        cmd_exec(&cmd);
        thread::sleep(Duration::from_millis(800));

        // Finalize the request
        let cmd = close_request();
        cmd_exec(&cmd);
        thread::sleep(Duration::from_millis(800));

        // Restart Samba to kick off user
        cmd_exec("/sbin/restart smbd");
    }
}

pub fn poweroff() {
    cmd_exec("shutdown -h now");
}
