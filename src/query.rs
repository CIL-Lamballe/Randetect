pub static DELETE: &str = "
    SELECT username, ip
    FROM    logs
    WHERE    id > (    SELECT    MAX(id) - 2500
            FROM    logs
            WHERE    isdir = 0 )
        AND cmd = 'delete'
    AND time > (    SELECT    MAX(time)
            FROM    logs ) - 100
        ;";

pub static SUSPICIOUS: &str = "
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

     UNION

     SELECT    *
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
