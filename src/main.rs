use std::collections::HashMap;
use std::{thread, time};

mod alert;
mod parse;
mod query;

/// Loop delay in milliseconds
const TIME: u64 = 2_000;

fn main() {
    let duration = time::Duration::from_millis(TIME);

    let mut list: HashMap<String, parse::UserInfo> = HashMap::new();

    //    loop {
    let mut query = query::select(query::Type::Move);
    query.extend(query::select(query::Type::Delete));
    query.extend(query::select(query::Type::SuspiciousCwd));

    parse::log(query, &mut list);
    for user in list.iter() {
        let (name, info) = user;
        //println!("List= {:?}", user);
        println!("List= {:?}", info.get_behaviors());
        for beh in info.get_behaviors() {
            match beh {
                parse::Behavior::Suspicious(c) if *c >= 50 => println!("BAN {}", name),
                parse::Behavior::Misbehaving(s) => println!("{} moved the folder {}", name, *s),
                _ => (),
            }
        }
        // println!("{:?}",user.UserInfo);
    }

    // thread::sleep(duration);
    //    }
}
