use rusqlite::{params, Connection, Result};
use std::collections::HashMap;

//const DB: &str = "/var/log/synolog/.SMBXFERDB";
const DB: &str = "/home/antoine/RanDetect/.SMBXFERDB"; // For dev

#[derive(Debug)]
struct Log {
    username: String,
    ip: String,
}

pub fn suspicious_pairs() {}

pub fn suspicious_triplet() {}

pub fn huge_delete() -> Result<()> {
    let conn = Connection::open(DB).unwrap();
    println!("Connect");

    let mut stmt = conn
        .prepare(
            "SELECT	username, ip
	FROM	logs",
        )
        .unwrap();
    //	WHERE	id > (	SELECT	MAX(id) - 2500
    //			FROM	logs
    //			WHERE	isdir = 0 )
    //		AND cmd = 'delete'
    //		AND time > (	SELECT	MAX(time)
    //				FROM	logs ) - 3;",
    //      )
    //    .unwrap();
    println!("Prepare Query");
    let logs = stmt
        .query_map(params![], |row| {
            Ok(Log {
                username: row.get(0).unwrap(),
                ip: row.get(1).unwrap(),
            })
        })
        .unwrap();

    for i in logs {
        println!("Line: {:?}", i);
    }
    println!("Done");
    Ok(())
}

pub fn dir_move() {}
