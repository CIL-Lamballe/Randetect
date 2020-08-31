pub mod sms {
    use crate::{Cdtl, parse::UserInfo};
    use std::{fs::File, io::Write, time::SystemTime, process::Command};

    fn file(timestamp: &str, core: &str) -> String {
        let fname = format!("{}_sms.txt", timestamp);
        let mut file = File::create(&fname).unwrap();
        file.write(core.as_bytes()).unwrap();
        fname
    }

    fn timestamp() -> String {
        let now = format!("{:?}", SystemTime::now());
        let now = format!("{}{}", &now[21..31], &now[42..51]);
        //println!("{}", now);
        now
    }

    fn prepare(cdtl: &Cdtl, uname: &str, info: &UserInfo) -> (String, String) {
        let tstamp = timestamp();
        let text = format!("{};TEST Alert NAS\n{}\n{:?}\n", cdtl.smsusr, uname, info);
        //println!("{}", text);

        let fname = file(&tstamp, &text);

        let arg = format!(
            "open -u {},{} {}; put -O {} {}_sms.txt",
            cdtl.user,
            cdtl.pwd,
            cdtl.sys,
            cdtl.folder,
            tstamp
        );
        (format!("lftp -c \"{}\"", arg), fname)
    }

    pub fn send(cdtl: &Cdtl, uname: &str, info: &UserInfo) {
        let (arg, fname) = prepare(cdtl, uname, info);
        //println!("\narg:{}\n{}\n", arg, fname);
     //           let output = Command::new("bash")
     //               .arg("-c")
      //              .arg(arg)
     //               .output()
     //               .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

        // Debug
        //        println!("status: {}", output.status);
        //        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        //        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        std::fs::remove_file(fname);
    }
}

pub mod email {
    use crate::parse::UserInfo;
    use std::process::Command;

    const to: &str = "a.barthleemy@cil-lamballe.com";

    pub fn send(user: &str, info: &UserInfo, act: &str) {
        let ssmtp = "ssmtp ".to_string()
            + to
            + &format!(
                " <<< \"{}\"",
                format!(
                    "Subject: {}\n{}\n",
                    format!("{} - {}", user, act),
                    format!("{} performed {}\nDetail:\n{:?}", user, act, info)
                )
            );

        //println!("{}", ssmtp);

       // let output = Command::new("bash")
       //     .arg("-c")
       //     .arg(ssmtp)
       //     .output()
       //     .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

        // Debug
        //        println!("status: {}", output.status);
        //        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        //        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }
}
