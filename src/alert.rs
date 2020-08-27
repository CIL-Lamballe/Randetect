use std::process::{Command, Stdio};

pub mod sms {
    pub fn send() {}
}

pub mod email {

    fn create(subject: &str, text: &str){

        let mail = format!("Subject: {}\n{}", subject, text);
        //println!("{}", mail);

        let path = std::path::Path::new("email.txt");
        let mut file = match std::fs::File::create(&path) {
            Err(why) => panic!("Failed to create email: {}", std::error::Error::description(&why)),
            Ok(file) => file,
        };
  //      match file.write_all(header_body.as_bytes()) {
  //          Err(why) => panic!("Could not write to email.txt: {}", std::error::Error::description(&why)),
  //          Ok(_) => (),
  //      }
    }

    pub fn send() {

        let to = "a.barthleemy@cil-lamballe.com";

        create("Suspicious user", "A user have been using it badly it has been banned");

        let ssmtp = "ssmtp ".to_string() + to + " < email.txt";

        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg(ssmtp)
            .output()
            .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });

        // println!("status: {}", output.status);
        // println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        // println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    }
}
