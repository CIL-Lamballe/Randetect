#!/bin/bash
#
# Script Name: randetect.sh
#
# Author: Antoine BARTHELEMY, Idrisse KARAMI
# Date : 2020-08-03
#
# Description: The following script parse a NAS Synology log file called SMBXFERDB
#              and classify user activity into supicious or non-suspicious.
#              Suspicious IPs are blacklisted and sent to iptables for ban.
#
# Run Information: This script is run automatically as a deamon every start up from a crontab entry.
#
# Error Log: Any errors or output associated with the script can be found in ?(not yet specified)
#


# Globals
#SLDPATH='/var/log/synolog/'
SLDPATH='/home/antoine/SynologyNAS_RansomwareAnalyzer/'
SLDNAME='.SMBXFERDB'
RANGE=2000
BAN_LIMIT=50
ERROR_LOG="/var/log/randetect.log"
BAN=()


function synology_log_query_type1() {
	local IFS=$'\n'
	local XMIN=1
	local YMIN=3
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
}


function synology_log_query_type2() {
	local IFS=$'\n'
	local XMIN=1
	local YMIN=300
	QUERY=`sqlite3 ${SLDPATH}${SLDNAME} "
	SELECT	*
	FROM
		(
			SELECT	A.ip, A.username, A.filename,
				B.filesize AS wrotefilesize,
				A.cmd, A.time, B.cmd,
				B.time AS btime
			FROM
				(
					SELECT	*
					FROM	logs
					WHERE	id > (	SELECT	MAX(id) - $RANGE
							FROM	logs
							WHERE	isdir = 0 )
				) A,
				(
					SELECT	*
					FROM	logs
					WHERE	id > (	SELECT	MAX(id) - $RANGE
							FROM	logs
							WHERE	isdir = 0 )
				) B
			WHERE	A.isdir = 0 AND B.isdir = 0
				AND A.filename = B.filename
				AND A.cmd = 'create' AND B.cmd='write'
				AND A.time <= B.time AND (B.time - A.time) <= $XMIN
		) CWp, (
			SELECT	C.filename,
				D.filesize AS deletedfilesize,
				C.cmd,
				C.time AS ctime,
				D.cmd,	D.time
			FROM
				(
					SELECT	*
					FROM	logs
					WHERE	id > (	SELECT	MAX(id) - $RANGE
							FROM	logs
							WHERE	isdir = 0 )
				) C,
				(
					SELECT	*
					FROM	logs
					WHERE	id > (	SELECT	MAX(id) - $RANGE
							FROM	logs
							WHERE	isdir = 0 )
				) D
			WHERE	C.isdir = 0 AND D.isdir = 0
				AND C.filename = D.filename
				AND C.cmd = 'read' AND D.cmd = 'delete'
				AND C.time <= D.time AND (D.time - C.time) <= $YMIN
		) RDp
	WHERE	CWp.btime = RDp.ctime
		AND RDp.deletedfilesize <= CWp.wrotefilesize
		AND RDp.deletedfilesize > 0
	;"`
}


function synology_log_query_type3() {
	local IFS=$'\n'
	local TIME=100
	QUERY=`sqlite3 ${SLDPATH}${SLDNAME} "
	SELECT	ip
	FROM	logs
	WHERE	id > (	SELECT	MAX(id) - $RANGE
			FROM	logs
			WHERE	isdir = 0 )
		AND cmd = 'delete'
		AND time > (	SELECT	MAX(time)
				FROM	logs ) - $TIME
	;"`
}


function classify_ip() {
	local index=0
	if [[ "${BLACKLIST[@]}" =~ "$1" ]];
	then
		while [ $index -lt ${#BLACKLIST[@]} ]
		do
			if [ "${BLACKLIST[$index]}" = "$1" ]
			then
				((++COUNTER[$index]))
				if [ ${COUNTER[$index]} -ge $BAN_LIMIT ]
				then
					ban $1 $2
				fi
			fi
			((++index))
		done
	else
		BLACKLIST+=($1)
	fi
}


function parse_ip_from_query_type1() {
	for ip in ${QUERY}
	do
		classify_ip $ip
	done
}


function parse_ip_from_query_type2() {
	local IFS=$'\n'
	for line in ${QUERY}
	do
		filenameA=`echo $line | cut -d '|' -f3`
		filenameC=`echo $line | cut -d '|' -f9`
		if [ `echo $line | cut -d '|' -f4` -eq `echo $line | cut -d '|' -f10` ]
		then
			cksA=`cksum ${filenameA} 2>/dev/null`
			cksC=`cksum ${filenameC} 2>/dev/null`
			if [ $((${cksA[0]})) -ne $((${cksC[0]})) ]
			then
				classify_ip `echo $line | cut -d '|' -f1`
			fi
		else
			classify_ip `echo $line | cut -d '|' -f1`
		fi
	done
}


function ban() {
	# Here is the iptable ban
	if [[ ! "${BAN[@]}" =~ "$1" ]];
	then
		BAN+=($1)
	fi
}


function main() {
	synology_log_query_type1
	parse_ip_from_query_type1

	unset COUNTER
	synology_log_query_type2
	parse_ip_from_query_type2

	unset COUNTER
	synology_log_query_type3
	parse_ip_from_query_type1

	echo ${BAN[@]}
}


#while true; do main ; sleep 2; done
main
