pub mod sms {
    use crate::{parse::UserInfo, Cdtl};
    use std::{fs, fs::File, io::Write, process::Command, time::SystemTime};

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
        //        println!("{:?}", now);
        let now = digits(&now);
        //        println!("{:?}", now);

        let fname = format!("{}_sms.txt", now);
        //        println!("{:?}", fname);

        let mut file = File::create(&fname).unwrap();
        file.write(text.as_bytes()).unwrap();
        fname
    }

    pub fn format(user: &str, msg: String) -> String {
        format!("Alert NAS user: {} {}", user, msg)
    }

    pub fn send(cdtl: &Cdtl, text: &str) {
        // write down text in a file which is the sms to be sent
        println!("{}", text);
        let fname = file(text);

        // Format the command to send sms
        let arg = format!(
            "open -u {},{} {}; put -O {} {}",
            cdtl.user, cdtl.pwd, cdtl.sys, cdtl.folder, fname
        );
        let arg = format!("lftp -c \"{}\"", arg);

        println!("\n{:?}\n", arg);

//        let output = Command::new("bash")
//            .arg("-c")
//            .arg(arg)
//            .output()
//            .unwrap_or_else(|e| panic!("failed to execute process: {}", e));
//
//        // Debug
//        println!("status: {}", output.status);
//        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
//        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        fs::remove_file(fname).unwrap();
    }
}

pub mod email {
    use crate::parse::UserInfo;
    use std::process::Command;

    const TO: &str = "a.barthelemy@cil-lamballe.com";

    pub fn send(user: &str, info: &UserInfo, act: &str) {
        let ssmtp = "ssmtp ".to_string()
            + TO
            + &format!(
                " <<< \"{}\"",
                format!(
                    "Subject: {}\n{}\n",
                    format!("{} - {}", user, act),
                    format!("{} performed {}\nDetail:\n{:?}", user, act, info)
                )
            );

        // println!("{}", ssmtp);

        //        let output = Command::new("bash")
        //            .arg("-c")
        //            .arg(ssmtp)
        //            .output()
        //            .unwrap_or_else(|e| panic!("failed to execute process: {}", e));
        //
        //        // Debug
        //                println!("status: {}", output.status);
        //                println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        //                println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }
}
