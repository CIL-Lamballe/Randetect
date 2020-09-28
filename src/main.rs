mod alert;
mod nas;
mod parse;
mod query;

extern crate daemonize;
extern crate sys_info;

use alert::{email, sms};
use daemonize::Daemonize;
use parse::Behavior;
use query::Type;
use rusqlite::Connection;
use std::{collections::HashMap, env, fs::File, process, thread, time::Duration};

macro_rules! nas_shutdown {
    () => {
        &format!(
            "Alert NAS {} shutdown ! Because of too many suspicious activities !",
            sys_info::hostname().unwrap()
        )
    };
}

/// Samaba Log Database
const DB: &str = "/var/log/synolog/.SMBXFERDB";

/// Maximum of suspicious actions
pub const BAN_LIMIT: i32 = 30;

pub struct Cdtl {
    user: String,
    pwd: String,
    sys: String,
    folder: String,
    mailto: String,
}

/// Loop delay in milliseconds
const TIME: u64 = 800;

/// Get environment variable for lftp use
fn getenv(var: &str) -> String {
    match env::var(var) {
        Ok(v) => v,
        Err(e) => panic!("{} : {}", var, e),
    }
}

fn env_variables() -> Cdtl {
    let crdtl = getenv("CRDTL");
    Cdtl {
        user: crdtl[..10].to_string(),
        pwd: crdtl[10..18].to_string(),
        sys: getenv("TARGETSYS"),
        folder: getenv("FOLDER"),
        mailto: getenv("MAILTO"),
    }
}

fn daemonize() {
    let stdout = File::create("/tmp/randetect.out").unwrap();
    let stderr = File::create("/tmp/randetect.err").unwrap();

    let daemonize = Daemonize::new()
        .pid_file("/run/randetect.pid")
        .chown_pid_file(true)
        .working_directory("/tmp")
        .user("root")
        .group("daemon")
        .group(2)
        .umask(0o077)
        .stdout(stdout)
        .stderr(stderr)
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(_) => println!("Success, daemonized"),
        Err(e) => {
            eprintln!("Error, {}", e);
            process::exit(1)
        }
    }
}

fn main() {
    #[cfg(debug_assertions)]
    daemonize();

    let var: Cdtl = env_variables();

    nas::enable_firewall();

    let duration = Duration::from_millis(TIME);

    let conn = match Connection::open(DB) {
        Err(conn) => panic!("Could not reach/open database {} {}", DB, conn),
        Ok(conn) => conn,
    };
    let mut id = query::updated_id(&conn);
    let mut idsup = id;

    #[cfg(debug_assertions)]
    println!("loop: {}", TIME);

    loop {
        #[cfg(debug_assertions)]
        {
            println!("id: {:?}", id);
            println!("idsup: {:?}\n", idsup);
        }

        let mut list: HashMap<String, parse::UserInfo> = HashMap::new();
        let mut query = query::select(&conn, Type::Move, id);
        query.extend(query::select(&conn, Type::Delete, id));
        query.extend(query::select(&conn, Type::SuspiciousCwd, idsup));
        parse::log(query, &mut list);
        let mut shutdown = 0;

        #[cfg(debug_assertions)]
        println!("list {:?}\n-------", list);

        for user in &list {
            let (name, info) = user;
            for beh in info.get_behaviors() {
                match beh {
                    Behavior::Delete(c) if *c >= BAN_LIMIT => {
                        #[cfg(debug_assertions)]
                        {
                            println!("Alert NAS {} user: {} banned because of deleting +{} files from ip:{:?}"
                            , sys_info::hostname().unwrap(), name, c, info.get_ips());
                        }

                        nas::ban(info);
                        email::send(&var, &name, info, "delete");
                        sms::send(&var, &format!(
                                "Alert NAS {} user: {} banned because of deleting +{} files from ip:{:?}"
                                , sys_info::hostname().unwrap(), name, c, info.get_ips()));
                        id = query::updated_id(&conn);
                    }
                    Behavior::Suspicious(c) if *c >= BAN_LIMIT => {
                        #[cfg(debug_assertions)]
                        {
                            println!("Alert NAS {} user: {} banned because of suspicious activity +{} times from ip:{:?}"
                            , sys_info::hostname().unwrap(), name, c, info.get_ips());
                            println!("idsup: {:?}", idsup);
                        }

                        nas::ban(info);
                        shutdown += 1;
                        email::send(&var, &name, info, "Suspicious");
                        sms::send(&var, &format!(
                                "Alert NAS {} user: {} banned because of suspicious activity +{} times from ip:{:?}"
                                , sys_info::hostname().unwrap(), name, c, info.get_ips()));
                        idsup = query::updated_id(&conn);
                        id = query::updated_id(&conn);
                    }
                    Behavior::Move(_s) => {
                        email::send(&var, &name, info, "Move");
                        id = query::updated_id(&conn);
                    }
                    _ => (),
                }
            }
            if shutdown > 1 {
                sms::send(&var, nas_shutdown!());
                nas::poweroff();
            }
        }
        thread::sleep(duration);
    }
}
