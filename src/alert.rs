pub mod sms {
    use crate::{nas, Cdtl};
    use std::{fs, fs::File, io::Write, time::SystemTime};

    fn digits(s: &str) -> String {
        let mut digits = String::new();
        for c in s.chars() {
            if c.is_ascii_digit() {
                digits.push(c);
            }
        }
        digits
    }

    fn file(text: String) -> String {
        let now = format!("{:?}", SystemTime::now());
        //        println!("{:?}", now);
        let now = digits(&now);
        //        println!("{:?}", now);

        let fname = format!("{}_sms.txt", now);
        //        println!("{:?}", fname);

        let mut file = File::create(&fname).unwrap();
        file.write(text.as_bytes()).unwrap();
        fname
    }

    pub fn send(cdtl: &Cdtl, text: String) {
        // write down text in a file which is the sms to be sent
        //      println!("{}", text);
        let fname = file(text);

        // Format the command to send sms
        let arg = format!(
            "open -u {},{} {}; put -O {} {}",
            cdtl.user, cdtl.pwd, cdtl.sys, cdtl.folder, fname
        );
        let arg = format!("lftp -c \"{}\"", arg);
        //        println!("\n{:?}\n", arg);
        nas::cmd_exec(&arg);
        fs::remove_file(fname).unwrap();
    }
}

pub mod email {
    use crate::{nas, Cdtl, parse::UserInfo};

    pub fn send(cdtl: &Cdtl, user: &str, info: &UserInfo, act: &str) {
        let ssmtp = "ssmtp ".to_string()
            + &cdtl.mailto
            + &format!(
                " <<< \"{}\"",
                format!(
                    "Subject: {}\n{}\n",
                    format!("{} - {}", user, act),
                    format!("{} performed {}\nDetail:\n{:?}", user, act, info)
                )
            );
        // println!("{}", ssmtp);
        nas::cmd_exec(&ssmtp);
    }
}
