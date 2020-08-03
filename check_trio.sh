#!/bin/bash

IFS=$'\n'
#SLDPATH='/var/log/synolog/'
SLDPATH='/home/antoine/SynologyNAS_RansomwareAnalyzer/'
SLDNAME='.SMBXFERDB'
xmin=1
ymin=3
range=2000

QUERY=`sqlite3 ${SLDPATH}${SLDNAME} "
SELECT D.ip
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
						SELECT MAX(id) - $range
						FROM	logs
						WHERE	isdir = 0
					)
			) A,
			(
				SELECT	*
				FROM	logs
				WHERE	id > (
						SELECT	MAX(id) - $range
						FROM	logs
						WHERE	isdir = 0
					)
	     		) B
		WHERE	A.filename = B.filename
			AND A.cmd = 'create' AND B.cmd = 'write'
			AND createtime <= writetime AND (writetime - createtime) <= $xmin
	) CWp,
	(
		SELECT	*
		FROM	logs
		WHERE	isdir = 0 AND cmd = 'delete'
	) D
WHERE	CWp.writetime <= D.time
	AND (D.time - CWp.writetime) <= $ymin
	AND D.filesize <= CWp.wrotefilesize
;"`

BLACKLIST=()

for i in ${QUERY}
do
	echo $i
	
done
