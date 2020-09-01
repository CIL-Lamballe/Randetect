use crate::parse::UserInfo;

pub fn ban(user: &str, info: &UserInfo, attempt: i32) {
    println!("BAN of {} because he/she as been deleting {} files", user, attempt); // To put in a log file

}
