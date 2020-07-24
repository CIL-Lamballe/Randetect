#!/bin/bash

## Samba Log Database
# Path
SLDPATH='/home/antoine/SynologyNAS_RansomwareAnalyzer/'
# Name
SLDNAME='SMBXFERDB_test'


## Database format (line 1: Element name, line 2: Tuple default value) 
#         ______________________________________________________________________________________________________________________
# line 1 | id (int)    | time (int) | ip (text) | username (text) | cmd (text) | filesize (int) | filename (text) | isdir (int) |
#        |-------------|------------|-----------|-----------------|------------|----------------|-----------------|-------------|
# line 2 | primary key |    NULL    |   NULL    |     NULL        |    NULL    |     NULL       |      NULL       |     0       |
#        |_____________|____________|___________|_________________|____________|________________|_________________|_____________|


##
#sqlite3 ${SLDPATH}${SLDNAME} "SELECT username FROM logs WHERE username"
sqlite3 ${SLDPATH}${SLDNAME} "SELECT * FROM logs WHERE username LIKE '%pgik%'"
