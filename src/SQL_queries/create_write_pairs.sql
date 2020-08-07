SELECT A.ip, A.username, A.filename,
	B.filesize AS wrotefilesize,
	A.cmd, A.time, B.cmd,
	B.time AS btime
FROM	logs A, logs B
WHERE	A.isdir = 0 AND B.isdir = 0
	AND A.filename = B.filename
	AND A.cmd = 'create' AND B.cmd = 'write'
	AND A.time <= B.time AND (B.time - A.time) <= $xmin
;
