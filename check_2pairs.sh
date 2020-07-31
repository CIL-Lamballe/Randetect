#!/bin/bash

IFS=$'\n'
#SLDPATH='/var/log/synolog/'
SLDPATH='/home/antoine/SynologyNAS_RansomwareAnalyzer/'
SLDNAME='.SMBXFERDB'
xmin=1
ymin=30
range=2000

QUERY=`sqlite3 ${SLDPATH}${SLDNAME} "
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
;"`

for i in ${QUERY}
do
	filenameA=`echo $i | cut -d '|' -f3`
	filenameC=`echo $i | cut -d '|' -f9`
	if [ `echo $i | cut -d '|' -f4` -eq `echo $i | cut -d '|' -f10` ]
	then
		cksA=`cksum ${filenameA} &>/dev/null`
		cksC=`cksum ${filenameC} &>/dev/null`
		if [ $((${cksA[0]})) -ne $((${cksC[0]})) ]
			then
			printf "\nChecksum Suspect operation:\n$i\n"
		fi
	else
		printf "Filesize Suspect operation:\n$i\n"
	fi
done
