use std::{thread, time};
use std::collections::HashMap;

mod query;
mod parse;
mod alert;

/// Loop delay in milliseconds
const TIME: u64 = 2_000;

fn main() {

    let list: HashMap<String, parse::UserInfo> = HashMap::new();
    let duration = time::Duration::from_millis(TIME);

    //    loop {
    let mut query = query::select(query::Type::Move);
    query.extend(query::select(query::Type::Delete));
    query.extend(query::select(query::Type::SuspiciousCwd));

    let list = parse::log(query, list);
    for user in list.iter() {
        println!("List= {:?}", user);
    }

    thread::sleep(duration);
    //    }
}
