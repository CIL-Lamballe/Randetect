use rusqlite::{params, Connection, Result};

fn fmt_qdelete(id: i32, period: i32) -> String {
    // period 100
    // id 2500
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
    //id 2_500
    //period 3
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
      WHERE    id > {} AND isdir = 0
     ) A,
     (
      SELECT    *
      FROM    logs
      WHERE    id > {} AND isdir = 0
     ) B
     WHERE    A.filename = B.filename
     AND A.cmd = 'create' AND B.cmd = 'write'
     AND createtime <= writetime AND (writetime - createtime) <= 1
     ) CWp,
     (
      SELECT    *
      FROM    logs
      WHERE    isdir = 0 AND cmd = 'delete'
                AND id > {} AND isdir = 0
     ) D
     WHERE    CWp.writetime <= D.time
     AND (D.time - CWp.writetime) <= {}
     AND D.filesize <= CWp.wrotefilesize
    ;",
        id, id, id, period
    )
}

fn fmt_qsuspiciouscrwd(id: i32, period: i32) -> String {
    // 2_500
    // 3
    format!(
        "SELECT *
     FROM
     (
      SELECT    A.ip, A.username, A.filename,
      B.filesize AS wrotefilesize,
      A.cmd, A.time, B.cmd,
      B.time AS btime
      FROM
      (
       SELECT    *
       FROM    logs
       WHERE    id > {} AND isdir = 0
      ) A,
      (
       SELECT    *
       FROM    logs
       WHERE    id > {} AND isdir = 0
      ) B
      WHERE    A.isdir = 0 AND B.isdir = 0
     AND A.filename = B.filename
     AND A.cmd = 'create' AND B.cmd='write'
     AND A.time <= B.time AND (B.time - A.time) <= 1
     ) CWp, (
         SELECT    C.filename,
         D.filesize AS deletedfilesize,
         C.cmd,
         C.time AS ctime,
         D.cmd,    D.time
         FROM
         (
          SELECT    *
          FROM    logs
          WHERE    id > {} AND isdir = 0
         ) C,
         (
          SELECT    *
          FROM    logs
          WHERE    id > {} AND isdir = 0
         ) D
         WHERE    C.isdir = 0 AND D.isdir = 0
         AND C.filename = D.filename
         AND C.cmd = 'read' AND D.cmd = 'delete'
         AND C.time <= D.time AND (D.time - C.time) <= {}
         ) RDp
         WHERE    CWp.btime = RDp.ctime
         AND RDp.deletedfilesize <= CWp.wrotefilesize
         AND RDp.deletedfilesize > 0
         ;",
        id, id, id, id, period
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
    SuspiciousCrwd,
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

/// Retrieve SQL relations corresponding to given user action(qtype: Move | Delete | SuspiciousCwd
/// | SuspiciousCrwd)
pub fn select(conn: &Connection, qtype: Type, id: &i32) -> Vec<Log> {
    let mut stmt = {
        match qtype {
            Type::Delete => conn.prepare(&fmt_qdelete(*id, 100)).unwrap(),
            Type::SuspiciousCwd => conn.prepare(&fmt_qsuspiciouscwd(*id, 3)).unwrap(),
            Type::SuspiciousCrwd => conn.prepare(&fmt_qsuspiciouscrwd(*id, 3)).unwrap(),
            Type::Move => conn.prepare(&fmt_qmove(*id)).unwrap(),
        }
    };

    //  println!("query:{:?}", stmt);

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
        //      println!("here: {:?}", each);
        match each {
            Ok(t) => relation.push(t),
            Err(_e) => (),
        }
    }
    relation
}
