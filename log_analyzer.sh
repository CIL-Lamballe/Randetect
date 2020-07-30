#!/bin/bash


## Samba Log Database
# Path
SLDPATH='/home/antoine/SynologyNAS_RansomwareAnalyzer/'
# Name
SLDNAME='SMBXFERDB_test'


## Database Format (line 1: Element name, line 2: Tuple default value)
#         ______________________________________________________________________________________________________________________
# line 1 | id (int)    | time (int) | ip (text) | username (text) | cmd (text) | filesize (int) | filename (text) | isdir (int) |
#        |-------------|------------|-----------|-----------------|------------|----------------|-----------------|-------------|
# line 2 | primary key |    NULL    |   NULL    |     NULL        |    NULL    |     NULL       |      NULL       |     0       |
#        |_____________|__Unix time_|___________|_________________|____________|________________|_________________|_____________|


## SQL Query from Synology Database

# Pairs create and write within 1 second
#sqlite3 ${SLDPATH}${SLDNAME} "select A.id, A.filename, A.ip, A.username, A.cmd, B.cmd, A.time, B.time as btime from logs A, logs B where A.isdir=0 and B.isdir=0 and A.filename=B.filename and A.cmd='create' and B.cmd='write' and A.time<=B.time and (B.time-A.time)<=1"

# Pairs read/delete within y minutes
ymin=300
#sqlite3 ${SLDPATH}${SLDNAME} "select C.id, C.filename, C.ip, C.username, C.cmd, D.cmd, C.time as ctime, D.time from logs C, logs D where C.isdir=0 and D.isdir=0 and C.filename=D.filename and C.cmd='read' and D.cmd='delete' and C.time<=D.time and (D.time-C.time)<=$ymin"

# Check if write file and delete file has similarities name and same size to guess whether it is a ransomware
IFS=$'\n'

QUERY=`sqlite3 ${SLDPATH}${SLDNAME} "
select *
from
(
	select A.id, A.filename, B.filesize, A.ip, A.username, A.cmd, B.cmd, A.time, B.time as btime
	from logs A, logs B
	where A.isdir=0 and B.isdir=0 and A.filename=B.filename and A.cmd='create' and B.cmd='write'
	and A.time<=B.time and (B.time-A.time)<=1
) CWp,
(
	select C.id, C.filename, C.filesize, C.ip, C.username, C.cmd, D.cmd, C.time as ctime, D.time from
	logs C, logs D
	where C.isdir=0 and D.isdir=0 and C.filename=D.filename and C.cmd='read' and D.cmd='delete'
	and C.time<=D.time and (D.time-C.time)<=$ymin
) RDp
where CWp.btime=RDp.ctime"`

for i in ${QUERY}
do
	filenameA=`echo $i | cut -d '|' -f2`
	filenameC=`echo $i | cut -d '|' -f11`
	if [ ${filenameA} = ${filenameC} ]
	then
		echo "Same name, here should check similar name instead"
		if [ `echo $i | cut -d '|' -f3` -eq `echo $i | cut -d '|' -f12` ]
		then
			echo "Same size"
			sizeA=`sha1sum ${filenameA}`
			sizeC=`sha1sum ${filenameC}`
			if [ $((${sizeA[0]})) -eq $((${sizeC[0]})) ]
			then
				echo "Same checksum"
			else
				echo "Illegal instruction"
			fi
		fi
	else
		echo "Illegal instruction"
	fi
	# Debug
	#	printf "$i\n"
	#	printf "\n\n$i\n\n"
done
