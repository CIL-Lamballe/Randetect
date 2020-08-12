use std::{thread, time};



fn main() {
    let duration = time::Duration::from_millis(2000);
    loop {
        println!("Hello, world!");
        thread::sleep(duration);
    }
}
