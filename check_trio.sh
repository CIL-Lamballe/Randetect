#!/bin/bash
IFS=$'\n'
#SLDPATH='/var/log/synolog/'
SLDPATH='/home/antoine/SynologyNAS_RansomwareAnalyzer/'
SLDNAME='.SMBXFERDB'
XMIN=1
YMIN=3
RANGE=2000
BAN_LIMIT=50
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
;"`
BLACKLIST=()
COUNTER=()

for ip in ${QUERY}
do
	#echo $ip
	if [[ "${BLACKLIST[@]}" =~ "$ip" ]];
	then
		index=0
		while [ $index -lt ${#BLACKLIST[@]} ]
		do
			if [ "${BLACKLIST[$index]}" = "$ip" ]
			then
				((++COUNTER[$index]))
				if [ ${COUNTER[$index]} -ge $BAN_LIMIT ]
				then
					echo BAN: $ip, ${COUNTER[$index]}
				fi
				#echo "BAN: index:" $index "COUNTER:${COUNTER[$index]}"
			fi
			((++index))
		done
	else
		BLACKLIST+=($ip)
	fi
done
