#!/bin/bash

IFS=$'\n'
#SLDPATH='/var/log/synolog/'
SLDPATH='/home/antoine/SynologyNAS_RansomwareAnalyzer/'
SLDNAME='.SMBXFERDB'

xmin=1
ymin=300
range=2000

QUERY=`sqlite3 ${SLDPATH}${SLDNAME} "
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
				select max(id) - $range
				from logs
				)
			and isdir=0
	     ) A,
		(
			select *
			from logs
			where id > (
				select max(id) - $range
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
where CWp.writetime <= D.time
	and (D.time - CWp.writetime) <= $ymin
	and D.filesize <= CWp.wrotefilesize"`


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
