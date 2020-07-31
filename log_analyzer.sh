#!/bin/bash


## Samba Log Database
# Path
#SLDPATH='/var/log/synolog/'
SLDPATH='/home/antoine/SynologyNAS_RansomwareAnalyzer/'
# Name
SLDNAME='.SMBXFERDB'


## Database Format (line 1: Element name, line 2: Tuple default value)
#         ______________________________________________________________________________________________________________________
# line 1 | id (int)    | time (int) | ip (text) | username (text) | cmd (text) | filesize (int) | filename (text) | isdir (int) |
#        |-------------|------------|-----------|-----------------|------------|----------------|-----------------|-------------|
# line 2 | primary key |    NULL    |   NULL    |     NULL        |    NULL    |     NULL       |      NULL       |     0       |
#        |_____________|__Unix time_|___________|_________________|____________|________________|_________________|_____________|


## SQL Query from Synology Database

# Pairs create and write within x seconds
#Time between a create and a write operation on a same file
xmin=1
#sqlite3 ${SLDPATH}${SLDNAME} "select A.ip, A.username, A.filename, B.filesize as wrotefilesize, A.cmd, A.time, B.cmd, B.time as btime from logs A, logs B where A.isdir=0 and B.isdir=0 and A.filename=B.filename and A.cmd='create' and B.cmd='write' and A.time<=B.time and (B.time-A.time)<=$xmin"

# Pairs read/delete within y seconds
#Time between a read and a delete operation on a same file
ymin=300
#sqlite3 ${SLDPATH}${SLDNAME} "select C.filename, D.filesize as deletedfilesize, C.cmd, C.time as ctime, D.cmd, D.time from logs C, logs D where C.isdir=0 and D.isdir=0 and C.filename=D.filename and C.cmd='read' and D.cmd='delete' and C.time<=D.time and (D.time-C.time)<=$ymin"

# Check if write file and delete file has similarities name and same size to guess whether it is a ransomware
IFS=$'\n'

#QUERY=`sqlite3 ${SLDPATH}${SLDNAME} "
#select *
#from
#(
#	select A.ip, A.username, A.filename, B.filesize as wrotefilesize, A.cmd, A.time, B.cmd, B.time as btime
#	from logs A, logs B
#	where A.isdir=0 and B.isdir=0 and A.filename=B.filename and A.cmd='create' and B.cmd='write'
#	and A.time<=B.time and (B.time-A.time)<=$xmin
#) CWp,
#(
#	select C.filename, D.filesize as deletedfilesize, C.cmd, C.time as ctime, D.cmd, D.time
#	from logs C, logs D
#	where C.isdir=0 and D.isdir=0 and C.filename=D.filename and C.cmd='read' and D.cmd='delete'
#	and C.time<=D.time and (D.time-C.time)<=$ymin
#) RDp
#where CWp.btime=RDp.ctime and RDp.deletedfilesize<=CWp.wrotefilesize and RDp.deletedfilesize>0"`

# Last 2000
#sqlite3 .SMBXFERDB "select * from logs where id>(select max(id)-2000 from logs)"

QUERY2=`sqlite3 ${SLDPATH}${SLDNAME} "
select *
from
(
	select A.ip, A.username, A.filename,
		B.filesize as wrotefilesize, A.cmd,
		A.time as createtime, B.cmd,
		B.time as writetime
	from (
		select *
		from logs
		where id > (
				select max(id) - 2000
				from logs
				)
			and isdir=0
	     ) A,
		(
			select *
			from logs
			where id > (
				select max(id) - 2000
				from logs
					)
				and isdir=0
	     ) B
	where A.filename = B.filename
		and A.cmd = 'create' and B.cmd = 'write'
		and createtime <= writetime and (writetime - createtime) <= $xmin
) CWp,
(
	select *
	from logs
	where isdir = 0 and cmd = 'delete'
) D
# Here is the folow to filter properly
where CWp.writetime=D.time and RDp.deletedfilesize<=CWp.wrotefilesize and RDp.deletedfilesize>0"`

#	and C.time<=D.time and (D.time-C.time)<=$ymin

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
