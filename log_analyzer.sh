#!/bin/bash

IFS=$'\n'
#SLDPATH='/var/log/synolog/'
SLDPATH='/home/antoine/SynologyNAS_RansomwareAnalyzer/'
SLDNAME='.SMBXFERDB'

xmin=1
ymin=300
range=2000

QUERY=`sqlite3 ${SLDPATH}${SLDNAME} "
SELECT
	*
FROM
	(
		SELECT
			A.ip, A.username, A.filename,
			B.filesize as wrotefilesize, A.cmd,
			A.time as createtime, B.cmd,
			B.time as writetime
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
			A.filename = B.filename
			AND A.cmd = 'create' AND B.cmd = 'write'
			AND createtime <= writetime AND (writetime - createtime) <= $xmin
	) CWp,
	(
		SELECT
			*
		FROM
			logs
		WHERE
			isdir = 0 AND cmd = 'delete'
	) D
WHERE
	CWp.writetime <= D.time
	AND (D.time - CWp.writetime) <= $ymin
	AND D.filesize <= CWp.wrotefilesize
;"`


#for i in ${QUERY}
#do
#	filenameA=`echo $i | cut -d '|' -f3`
#	filenameC=`echo $i | cut -d '|' -f9`
#	if [ `echo $i | cut -d '|' -f4` -eq `echo $i | cut -d '|' -f10` ]
#	then
#		cksA=`cksum ${filenameA} &>/dev/null`
#		cksC=`cksum ${filenameC} &>/dev/null`
#		if [ $((${cksA[0]})) -ne $((${cksC[0]})) ]
#		then
#			printf "\nChecksum Suspect operation:\n$i\n"
#		fi
#	else
#		printf "Filesize Suspect operation:\n$i\n"
#	fi
#done
