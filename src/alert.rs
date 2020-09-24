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

    fn file(text: &str) -> String {
        let now = format!("{:?}", SystemTime::now());

        #[cfg(debug_assertions)]
        println!("{:?}", now);

        let now = digits(&now);

        #[cfg(debug_assertions)]
        println!("{:?}", now);

        let fname = format!("{}_sms.txt", now);

        #[cfg(debug_assertions)]
        println!("{:?}", fname);

        let mut file = File::create(&fname).unwrap();
        file.write(text.as_bytes()).unwrap();
        fname
    }

    pub fn send(cdtl: &Cdtl, text: &str) {
        /// write down text in a file which is the sms to be sent
        #[cfg(debug_assertions)]
        println!("{}", text);

        let fname = file(text);

        /// Format the command to send sms
        let arg = format!(
            "open -u {},{} {}; put -O {} {}",
            cdtl.user, cdtl.pwd, cdtl.sys, cdtl.folder, fname
        );
        let arg = format!("lftp -c \"{}\"", arg);

        #[cfg(debug_assertions)]
        println!("\n{:?}\n", arg);

        nas::cmd_exec(&arg);
        fs::remove_file(fname).unwrap();
    }
}

pub mod email {
    use crate::{nas, parse::UserInfo, Cdtl};

    pub fn send(cdtl: &Cdtl, user: &str, info: &UserInfo, act: &str) {
        let ssmtp = "ssmtp ".to_string()
            + &cdtl.mailto
            + &format!(
                " <<< \'{}\'",
                format!(
                    "Subject: {}\n{}\n",
                    format!("{} - {}", user, act),
                    format!("{} performed {}\nDetail:\n{:?}", user, act, info)
                )
            );

        #[cfg(debug_assertions)]
        println!("{}", ssmtp);

        nas::cmd_exec(&ssmtp);
    }
}
