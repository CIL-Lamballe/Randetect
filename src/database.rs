use rusqlite::{Connection, Result, NO_PARAMS};
use std::collections::HashMap;

#[derive(Debug)]
struct Log {
    username: String,
    ip: String,
}

pub fn suspicious_pairs() {}

pub fn suspicious_triplet() {}

pub fn huge_delete(db: &str) -> Result<()> {
    let conn = Connection::open(db)?;
    println!("Connect");

    let mut stmt = conn
        .prepare(
            "SELECT	username, ip
	FROM	logs
	WHERE	id > (	SELECT	MAX(id) - 2000
			FROM	logs
			WHERE	isdir = 0 )
		AND cmd = 'delete'
		AND time > (	SELECT	MAX(time)
				FROM	logs ) - 3;",
        )
        .unwrap();
    println!("Prepare Query");
    let logs = stmt.query_map(NO_PARAMS, |row| {
        Ok(Log {
            username: row.get(0)?,
            ip: row.get(1)?,
        })
    })?;

    println!("Ok Query");

    for log in logs {
        println!("Line: {:?}", log);
    }
    Ok(())
}

pub fn dir_move() {}
