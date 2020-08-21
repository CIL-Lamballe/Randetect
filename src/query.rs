use rusqlite::{params, Connection, Result};

//const DB: &str = "/var/log/synolog/.SMBXFERDB";
const DB: &str = "/home/antoine/RanDetect/.SMBXFERDB"; // For dev

#[derive(Debug)]
pub struct Log {
    username: String,
    ip: String,
    dir: Result<String>,
}

pub static DELETE: &str = "
    SELECT username, ip
    FROM   logs
    WHERE  id > ( SELECT MAX(id) - 2500
                  FROM logs
                  WHERE isdir = 0 )
           AND cmd = 'delete'
           AND time > ( SELECT MAX(time)
            FROM logs ) - 100
        ;";

pub static SUSPICIOUS_CWD: &str = "
    SELECT D.username, D.ip
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
      WHERE    id > (
          SELECT MAX(id) - 2500
          FROM    logs
          WHERE    isdir = 0
          )
     ) A,
     (
      SELECT    *
      FROM    logs
      WHERE    id > (
          SELECT    MAX(id) - 2500
          FROM    logs
          WHERE    isdir = 0
          )
     ) B
     WHERE    A.filename = B.filename
     AND A.cmd = 'create' AND B.cmd = 'write'
     AND createtime <= writetime AND (writetime - createtime) <= 1
     ) CWp,
     (
      SELECT    *
      FROM    logs
      WHERE    isdir = 0 AND cmd = 'delete'
     ) D
     WHERE    CWp.writetime <= D.time
     AND (D.time - CWp.writetime) <= 3
     AND D.filesize <= CWp.wrotefilesize
    ;";

pub static SUSPICIOUS_CRWD: &str = "
     SELECT *
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
       WHERE    id > (    SELECT    MAX(id) - 2500
           FROM    logs
           WHERE    isdir = 0 )
      ) A,
      (
       SELECT    *
       FROM    logs
       WHERE    id > (    SELECT    MAX(id) - 2500
           FROM    logs
           WHERE    isdir = 0 )
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
          WHERE    id > (    SELECT    MAX(id) - 2500
              FROM    logs
              WHERE    isdir = 0 )
         ) C,
         (
          SELECT    *
          FROM    logs
          WHERE    id > (    SELECT    MAX(id) - 2500
              FROM    logs
              WHERE    isdir = 0 )
         ) D
         WHERE    C.isdir = 0 AND D.isdir = 0
         AND C.filename = D.filename
         AND C.cmd = 'read' AND D.cmd = 'delete'
         AND C.time <= D.time AND (D.time - C.time) <= 300
         ) RDp
         WHERE    CWp.btime = RDp.ctime
         AND RDp.deletedfilesize <= CWp.wrotefilesize
         AND RDp.deletedfilesize > 0
         ;";

pub static MOVE: &str = "
    SELECT username, ip, filename
    FROM logs
    WHERE id > ( SELECT MAX(id) - 2500
            FROM logs )
        AND cmd = 'move'
        AND isdir = 1
    ;";

pub fn select(stmt: &str) {
    let conn = Connection::open(DB).unwrap();
    println!("Connect");
    let mut stmt = conn.prepare(stmt).unwrap();
    let logs = stmt
        .query_map(params![], |row| {
            Ok(Log {
                username: row.get(0).unwrap(),
                ip: row.get(1).unwrap(),
                dir: row.get(2),
            })
        })
        .unwrap();
    //     for i in logs {
    //      println!("Here treat log: Line: {:?}", i);
    //}
}
