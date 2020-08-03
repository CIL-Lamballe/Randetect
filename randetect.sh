#!/bin/bash
#
# Script Name: randetect.sh
#
# Author: Antoine BARTHELEMY, Idrisse KARAMI
# Date : 2020-08-03
#
# Description: The following script parse an SQL query from a NAS Synology log file called SMBXFERDB
#              and classify user activity into supicious or non-suspicious.
#              Suspicious IPs are blacklisted and send to iptables for ban.
#
# Run Information: This script is run automatically as a deamon every start up from a crontab entry.
#
# Error Log: Any errors or output associated with the script can be found in ?(not yet specified)
#

# Globals
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

function parse_ip_from_query() {
	#echo $ip
	if [[ "${BLACKLIST[@]}" =~ "$1" ]];
	then
		index=0
		while [ $index -lt ${#BLACKLIST[@]} ]
		do
			if [ "${BLACKLIST[$index]}" = "$1" ]
			then
				((++COUNTER[$index]))
				if [ ${COUNTER[$index]} -ge $BAN_LIMIT ]
				then
					echo BAN: $1, ${COUNTER[$index]}
				fi
				#echo "BAN: index:" $index "COUNTER:${COUNTER[$index]}"
			fi
			((++index))
		done
	else
		BLACKLIST+=($1)
	fi
}

function main() {
	for ip in ${QUERY}
	do
		#echo $ip
		parse_ip_from_query $ip
	done
}

main
