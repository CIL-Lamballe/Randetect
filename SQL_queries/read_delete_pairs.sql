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
;
