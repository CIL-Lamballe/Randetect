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
COUNTER=()

for i in ${QUERY}
do
	echo $i
	if [[ "${BLACKLIST[@]}" =~ "$i" ]];
	then
		count=0
	#	echo ${#BLACKLIST[@]}
		while [ $count -lt ${#BLACKLIST[@]} ]
		do
	#		printf "\n${BLACKLIST[$count]}\n"
			if [ "${BLACKLIST[$count]}" = "$i" ]
			then
				echo "BAN: index:" $count
			fi
			((++count))
		done
	    # whatever you want to do when array contains value
	else
		BLACKLIST+=($i)
	fi
done
