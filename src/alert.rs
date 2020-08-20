use std::fs::File;

pub mod sms {
    pub fn send() {}
}

pub mod email {
    pub fn send() {
        let f = File::open("email.txt");

        let f = match f {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("email.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error => panic!("Problem opening the file: {:?}", other_error),
            },
        };
    }
}
