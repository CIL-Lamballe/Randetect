use rusqlite::{params, Connection, Result};
use std::collections::HashMap;

//const DB: &str = "/var/log/synolog/.SMBXFERDB";
const DB: &str = "/home/antoine/RanDetect/.SMBXFERDB"; // For dev

#[derive(Debug)]
struct Log {
    username: String,
    ip: String,
    dir: String,
}

pub fn suspicious_pairs() {}

pub fn suspicious_triplet() {}

pub fn huge_delete() -> Result<()> {
    let conn = Connection::open(DB).unwrap();
    println!("Connect");

    let mut stmt = conn
        .prepare(
            "SELECT	username, ip
	FROM	logs
    	WHERE	id > (	SELECT	MAX(id) - 2500
    			FROM	logs
    			WHERE	isdir = 0 )
    		AND cmd = 'delete'
    		AND time > (	SELECT	MAX(time)
    				FROM	logs ) - 100000;", // Values to be changed, here just for tests
        )
        .unwrap();
    let logs = stmt
        .query_map(params![], |row| {
            Ok(Log {
                username: row.get(0).unwrap(),
                ip: row.get(1).unwrap(),
                dir: String::new()
            })
        })
        .unwrap();

    for i in logs {
        println!("Here treat log: Line: {:?}", i);
    }
    Ok(())
}

pub fn dir_move() {}
