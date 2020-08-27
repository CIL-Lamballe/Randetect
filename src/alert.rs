use std::process::Command;

pub mod sms {
    pub fn send() {}
}

pub mod email {

    pub fn send() {
            let output = std::process::Command::new("touch")
                     .arg("Hello")
                     .output()
                     .expect("Failed to execute command");
           // println!("{:?}", output);
    }
}
