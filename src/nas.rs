use crate::parse::UserInfo;

pub fn ban(user: &str, info: &UserInfo, attempt: i32) {
    println!("BAN of {} because he/she as been deleting {} files", user, attempt); // To put in a log file

    for ip in info.get_ips().iter() {
        let iptables = "iptables -A INPUT -s ".to_string() + ip + " -j DROP";
       // println!("{}", iptables);
    }

  //  let output = Command::new("bash")
  //      .arg("-c")
  //      .arg(iptables)
   //     .output()
   //     .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    // Debug
   // println!("status: {}", output.status);
   // println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
   // println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

}

pub fn poweroff() {
  //  let output = Command::new("bash")
  //      .arg("-c")
  //      .arg("shutdown -h now")
  //      .output()
  //      .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

   // Debug should send sms failed to poweroff and send when powering off
   // println!("status: {}", output.status);
   // println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
   // println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

}
