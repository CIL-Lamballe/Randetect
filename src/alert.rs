use std::process::{Command, Stdio};

pub mod sms {
    pub fn send() {}
}

pub mod email {
    pub fn send() {
        let to = "a.barthleemy@cil-lamballe.com";

        let ssmtp = "ssmtp ".to_string() + to + &format!(" <<< \"{}\"", format!("Subject: {}\n{}\n", "Suspicious user", "A user have been using it badly it has been banned"));
        println!("{}", ssmtp);
        let output = std::process::Command::new("bash")
            .arg("-c")
            .arg(ssmtp)
            .output()
            .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

        // Debug
          println!("status: {}", output.status);
          println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
          println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }
}
