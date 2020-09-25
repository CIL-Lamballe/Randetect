use rusqlite::{params, Connection, Result};

fn fmt_qdelete(id: i32, period: i32) -> String {
    format!(
        "SELECT username, ip
             FROM   logs
             WHERE  id > {} AND isdir = 0
             AND cmd = 'delete'
           AND time > ( SELECT MAX(time)
            FROM logs ) - {}
        ;",
        id, period
    )
}

fn fmt_qsuspiciouscwd(id: i32, period: i32) -> String {
    format!(
        "SELECT D.username, D.ip
             FROM
            (
     SELECT    A.ip, A.username, A.filename,
     B.filesize as wrotefilesize, A.cmd,
     A.time as createtime, B.cmd,
     B.time as writetime
     FROM
     (
      SELECT    *
      FROM    logs
      WHERE    id > {} AND time > strftime('%s', 'now') - 5 * 60 AND isdir = 0
     ) A,
     (
      SELECT    *
      FROM    logs
      WHERE    id > {} AND time > strftime('%s', 'now') - 5 * 60 AND isdir = 0
     ) B
     WHERE    A.filename = B.filename
     AND A.cmd = 'create' AND B.cmd = 'write'
     AND createtime <= writetime AND (writetime - createtime) <= 1
     ) CWp,
     (
      SELECT    *
      FROM    logs
      WHERE    isdir = 0 AND cmd = 'delete'
                AND id > {} AND time > strftime('%s', 'now') - 5 * 60 AND isdir = 0
     ) D
     WHERE    CWp.writetime <= D.time
     AND (D.time - CWp.writetime) <= {}
     AND D.filesize <= CWp.wrotefilesize
    ;",
        id, id, id, period
    )
}

fn fmt_qmove(id: i32) -> String {
    format!(
        "SELECT username, ip, filename
             FROM logs
             WHERE id > {}
             AND cmd = 'move'
             AND isdir = 1;",
        id
    )
}

#[derive(Debug)]
struct Id {
    id: i32,
}

impl Id {
    fn get_id(&self) -> i32 {
        self.id
    }
}

static MAXID: &str = "SELECT MAX(id) FROM logs;";

pub fn updated_id(conn: &Connection) -> i32 {
    let mut stmt = conn.prepare(MAXID).unwrap();

    let max = stmt
        .query_map(params![], |row| {
            Ok(Id {
                id: row.get(0).unwrap(),
            })
        })
        .unwrap();
    let mut ret: i32 = 0;
    for m in max {
        ret = m.unwrap().get_id();
    }
    ret
}

#[derive(Copy, Clone, Debug)]
pub enum Type {
    Delete,
    SuspiciousCwd,
    Move,
}

#[derive(Debug)]
pub struct Log {
    username: String,
    ip: String,
    dir: Result<String>,
    kind: Type,
}

impl Log {
    pub fn get_username(&self) -> String {
        String::from(&self.username)
    }

    pub fn get_ip(&self) -> String {
        String::from(&self.ip)
    }

    pub fn get_dir(&self) -> String {
        match &self.dir {
            Ok(f) => String::from(f),
            Err(_e) => String::from("empty"),
        }
    }

    pub fn get_kind(&self) -> Type {
        self.kind
    }
}

/// Retrieve SQL relations corresponding to given user `action(qtype: Move | Delete | SuspiciousCwd)`
pub fn select(conn: &Connection, qtype: Type, id: i32) -> Vec<Log> {
    let mut stmt = {
        match qtype {
            Type::Delete => conn.prepare(&fmt_qdelete(id, 3)).unwrap(),
            Type::SuspiciousCwd => conn.prepare(&fmt_qsuspiciouscwd(id, 5)).unwrap(),
            Type::Move => conn.prepare(&fmt_qmove(id)).unwrap(),
        }
    };

    //    #[cfg(debug_assertions)]
    //    println!("query stmt:{:?}", stmt);

    let logs = stmt
        .query_map(params![], |row| {
            Ok(Log {
                username: row.get(0).unwrap(),
                ip: row.get(1).unwrap(),
                dir: row.get(2),
                kind: qtype,
            })
        })
        .unwrap();

    let mut relation: Vec<Log> = Vec::new();
    for each in logs {
     //   #[cfg(debug_assertions)]
     //   println!("eachlog: {:?}", each);

        match each {
            Ok(t) => relation.push(t),
            Err(_e) => (),
        }
    }
    relation
}
