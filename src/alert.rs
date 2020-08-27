use std::process::{Command, Stdio};

pub mod sms {
    pub fn send() {}
}

pub mod email {
    use std::error::Error;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;
    use std::process::Command;

    fn create(subject: &str, text: &str) {
        let mail = format!("Subject: {}\n{}\n", subject, text);
        //println!("{}", mail);

        let path = Path::new("email.txt");
        let mut file = match std::fs::File::create(&path) {
            Err(why) => panic!("Failed to create email: {}", Error::description(&why)),
            Ok(file) => file,
        };
        match file.write_all(mail.as_bytes()) {
            Err(why) => panic!("Could not write to email.txt: {}", Error::description(&why)),
            Ok(_) => (),
        }
    }

    pub fn send() {
        let to = "a.barthleemy@cil-lamballe.com";

        create(
            "Suspicious user",
            "A user have been using it badly it has been banned",
        );

        let ssmtp = "ssmtp ".to_string() + to + " < email.txt";

        let output = Command::new("sh")
            .arg("-c")
            .arg(ssmtp)
            .output()
            .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

        // Debug
        //  println!("status: {}", output.status);
        //  println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        //  println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }
}
