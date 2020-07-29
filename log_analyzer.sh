#!/bin/bash


##                        GENERAL ARCHITECTURE
##
##         Crontab -|
##                  |-> ./ransomware_analyzer * checksum
##
##
##
##
##
##


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
#sqlite3 ${SLDPATH}${SLDNAME} "SELECT distinct username, ip FROM logs ORDER BY username ASC"


sqlite3 ${SLDPATH}${SLDNAME} "select A.filename, A.ip, A.username, A.cmd, B.cmd, A.time, B.time from logs A, logs B where A.filename=B.filename and A.cmd='create' and B.cmd='write' and A.time<=B.time and (B.time-A.time)<=1"
