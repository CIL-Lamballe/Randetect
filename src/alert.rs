pub mod sms {
    pub fn send() {}
}

pub mod email {

    pub fn send() -> Result<std::fs::File, std::io::Error> {
        let f = std::fs::File::open("email.txt")?;
        Ok(f)
    }
}
