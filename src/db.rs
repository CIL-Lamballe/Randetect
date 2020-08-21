//const DB: &str = "/var/log/synolog/.SMBXFERDB";
//const DB: &str = "/home/antoine/RanDetect/.SMBXFERDB"; // For dev

//#[derive(Debug)]
//struct Log {
//    username: String,
//    ip: String,
//    dir: String,
//}
//
//pub mod query {
//
pub static delete: &str = "
			SELECT username, ip
			FROM	logs
    			WHERE	id > (	SELECT	MAX(id) - 2500
    					FROM	logs
    					WHERE	isdir = 0 )
    				AND cmd = 'delete'
    				AND time > (	SELECT	MAX(time)
    						FROM	logs ) - 100
			;"; // Should be 100
//
//    pub fn select() {
//        let conn = rusqlite::Connection::open(DB).unwrap();
//        println!("Connect");
//
//        let mut stmt = conn.prepare(query::huge_delete::stmt).unwrap();
//        let logs = stmt
//            .query_map(params![], |row| {
//                Ok(Log {
//                    username: row.get(0).unwrap(),
//                    ip: row.get(1).unwrap(),
//                    dir: String::new(),
//                })
//            })
//            .unwrap();
//
//        for i in logs {
//            println!("Here treat log: Line: {:?}", i);
//        }
//    }
//}
//
//pub fn suspicious_pairs() {}
//

pub static suspicious: &str = "
		SELECT D.username, D.ip
		FROM
		(
		 SELECT	A.ip, A.username, A.filename,
		 B.filesize as wrotefilesize, A.cmd,
		 A.time as createtime, B.cmd,
		 B.time as writetime
		 FROM
		 (
		  SELECT	*
		  FROM	logs
		  WHERE	id > (
			  SELECT MAX(id) - $RANGE
			  FROM	logs
			  WHERE	isdir = 0
			  )
		 ) A,
		 (
		  SELECT	*
		  FROM	logs
		  WHERE	id > (
			  SELECT	MAX(id) - $RANGE
			  FROM	logs
			  WHERE	isdir = 0
			  )
		 ) B
			 WHERE	A.filename = B.filename
			 AND A.cmd = 'create' AND B.cmd = 'write'
			 AND createtime <= writetime AND (writetime - createtime) <= $XMIN
			 ) CWp,
		 (
		  SELECT	*
		  FROM	logs
		  WHERE	isdir = 0 AND cmd = 'delete'
		 ) D
			 WHERE	CWp.writetime <= D.time
			 AND (D.time - CWp.writetime) <= $YMIN
			 AND D.filesize <= CWp.wrotefilesize
			 ;";
