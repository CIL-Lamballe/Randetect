pub mod sms {
    use crate::Cdtl;
    use std::fs::File;
    use std::io::Write;

    fn file(timestamp: &str, core: &str) -> String {
        let fname = format!("{}_sms.txt", timestamp);
        let mut file = File::create(&fname).unwrap();
        file.write(core.as_bytes()).unwrap();
        fname
    }

    fn timestamp() -> String {
        let now = format!("{:?}", std::time::SystemTime::now());
        let now = format!("{}{}", &now[21..31], &now[42..51]);
        //println!("{}", now);
        now
    }

    fn prepare(cdtl: &Cdtl) -> (String, String) {
        let tstamp = timestamp();
        let text = format!("{};TEST Alert NAS new prg\n", cdtl.get_smsusr());
        //    println!("{}", text);

        let fname = file(&tstamp, &text);

        let arg = format!(
            "open -u {},{} {}; put -O {} {}_sms.txt",
            cdtl.get_user(),
            cdtl.get_pwd(),
            cdtl.get_sys(),
            cdtl.get_folder(),
            tstamp
        );
        (format!("lftp -c \"{}\"", arg), fname)
    }

    pub fn send(cdtl: &Cdtl) {
        let (arg, fname) = prepare(cdtl);
        println!("\narg:{}\n{}\n", arg, fname);
        //   let output = std::process::Command::new("bash")
        //       .arg("-c")
        //       .arg(arg)
        //       .output()
        //       .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

        // Debug
        //        println!("status: {}", output.status);
        //        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        //        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        std::fs::remove_file(fname);
    }
}

pub mod email {
    const to: &str = "a.barthleemy@cil-lamballe.com";

    pub fn send(user: &str, info: &crate::parse::UserInfo, act: &str) {
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

        println!("{}", ssmtp);

        let output = std::process::Command::new("bash")
            .arg("-c")
            .arg(ssmtp)
            .output()
            .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

        // Debug
        //        println!("status: {}", output.status);
        //        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        //        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }
}
