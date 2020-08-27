use std::process::{Command, Stdio};

pub mod sms {
    pub fn send() {}
}

pub mod email {

    pub fn send(email: &str, msg: &str) {
        //println!("{} {}", email, msg);
    }
}
