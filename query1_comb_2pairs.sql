-- First version of the selection matching 2 pairs
SELECT
	*
FROM
	(
		SELECT
			A.ip,
			A.username,
			A.filename,
			B.filesize AS wrotefilesize,
			A.cmd,
			A.time,
			B.cmd,
			B.time AS btime
		FROM
			logs A, logs B
		WHERE
			A.isdir = 0 AND B.isdir = 0
			AND A.filename = B.filename
			AND A.cmd = 'create' AND B.cmd='write'
			AND A.time <= B.time AND (B.time - A.time) <= $xmin
	) CWp, (
		SELECT
			C.filename,
			D.filesize AS deletedfilesize,
			C.cmd,
			C.time AS ctime,
			D.cmd,
			D.time
		FROM
			logs C, logs D
		WHERE
			C.isdir = 0 AND D.isdir = 0
			AND C.filename = D.filename
			AND C.cmd = 'read' AND D.cmd = 'delete'
			AND C.time <= D.time AND (D.time - C.time) <= $ymin
	) RDp
WHERE
	CWp.btime = RDp.ctime
	AND RDp.deletedfilesize <= CWp.wrotefilesize
	AND RDp.deletedfilesize > 0
;

-- Second version of the selection, using range:
SELECT
	*
FROM
	(
		SELECT
			A.ip,
			A.username,
			A.filename,
			B.filesize AS wrotefilesize,
			A.cmd,
			A.time,
			B.cmd,
			B.time AS btime
		FROM
			(
				SELECT
					*
				FROM
					logs
				WHERE
					id > (
						SELECT
							MAX(id) - $range
						FROM
							logs
						WHERE
							isdir = 0
					)
			) A,
			(
				SELECT
					*
				FROM
					logs
				WHERE
					id > (
						SELECT
							MAX(id) - $range
						FROM
							logs
						WHERE
							isdir = 0
					)
			) B
		WHERE
			A.isdir = 0 AND B.isdir = 0
			AND A.filename = B.filename
			AND A.cmd = 'create' AND B.cmd='write'
			AND A.time <= B.time AND (B.time - A.time) <= $xmin
	) CWp, (
		SELECT
			C.filename,
			D.filesize AS deletedfilesize,
			C.cmd,
			C.time AS ctime,
			D.cmd,
			D.time
		FROM
			(
				SELECT
					*
				FROM
					logs
				WHERE
					id > (
						SELECT
							MAX(id) - $range
						FROM
							logs
						WHERE
							isdir = 0
					)
			) C,
			(
				SELECT
					*
				FROM
					logs
				WHERE
					id > (
						SELECT
							MAX(id) - $range
						FROM
							logs
						WHERE
							isdir = 0
					)
			) D
		WHERE
			C.isdir = 0 AND D.isdir = 0
			AND C.filename = D.filename
			AND C.cmd = 'read' AND D.cmd = 'delete'
			AND C.time <= D.time AND (D.time - C.time) <= $ymin
	) RDp
WHERE
	CWp.btime = RDp.ctime
	AND RDp.deletedfilesize <= CWp.wrotefilesize
	AND RDp.deletedfilesize > 0
;
