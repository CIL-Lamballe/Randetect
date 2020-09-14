# Ransomware Detector for Synology NAS

An implementation of a naive ransomware detection algorithm running on Synology Network Attached Storage.

This project aims at building a deamon started at DSM boot time. The deamon will monitors the changes made to the filesystem by its clients, filtering it with a customizable set of rules and banning ips or powering off the NAS depending on criticity level.



---

## NAS Log Queries Relational Algebra

```
## Database Format (line 1: Element name, line 2: Tuple default value)
#         ______________________________________________________________________________________________________________________
# line 1 | id (int)    | time (int) | ip (text) | username (text) | cmd (text) | filesize (int) | filename (text) | isdir (int) |
#        |-------------|------------|-----------|-----------------|------------|----------------|-----------------|-------------|
# line 2 | primary key |    NULL    |   NULL    |     NULL        |    NULL    |     NULL       |      NULL       |     0       |
#        |_____________|__Unix time_|___________|_________________|____________|________________|_________________|_____________|
```

SQL Query from Synology Database details
Pairs read/delete within y seconds.
ymin: Time between a read and a delete operation on a same file


<img src="https://latex.codecogs.com/gif.latex?\small&space;\prod_{ip,&space;username}\sigma_{A.cmd\,=\,'create'&space;\;\cap\;&space;B.cmd\,=\,'write'&space;\;\cap\;&space;A.filename\,=\,B.filename&space;\;\cap\;&space;0\,\geq\,&space;B.time-A.time\,\leq\,&space;1}&space;\left&space;(&space;\rho_{A}&space;\left&space;(logs&space;\right&space;)&space;\times&space;\rho_{B}\left&space;(&space;logs&space;\right&space;)&space;\right&space;)" title="\small \prod_{ip, username}\sigma_{A.cmd\,=\,'create' \;\cap\; B.cmd\,=\,'write' \;\cap\; A.filename\,=\,B.filename \;\cap\; 0\,\geq\, B.time-A.time\,\leq\, 1} \left ( \rho_{A} \left (logs \right ) \times \rho_{B}\left ( logs \right ) \right )" />

Pairs create and write within x seconds.
xmin: Time between a create and a write operation on a same file.

<img src="https://latex.codecogs.com/gif.latex?\small&space;\prod_{ip,&space;username}\sigma_{C.cmd\,=\,'read'&space;\;\cap\;&space;D.cmd\,=\,'delete'&space;\;\cap\;&space;C.filename\,=\,D.filename&space;\;\cap\;&space;0\,\geq\,&space;D.time-C.time\,\leq\,&space;y,\:&space;y\,&space;\neq\,&space;0&space;}&space;\left&space;(&space;\rho_{C}&space;\left&space;(logs&space;\right&space;)&space;\times&space;\rho_{D}\left&space;(&space;logs&space;\right&space;)&space;\right&space;)" title="\small \prod_{ip, username}\sigma_{C.cmd\,=\,'read' \;\cap\; D.cmd\,=\,'delete' \;\cap\; C.filename\,=\,D.filename \;\cap\; 0\,\geq\, D.time-C.time\,\leq\, y,\: y\, \neq\, 0 } \left ( \rho_{C} \left (logs \right ) \times \rho_{D}\left ( logs \right ) \right )" />

Encrypting does not inherently make a file larger but most of the time it does, for this reason we randomly checksum the files been raised by the alert.



---

## Run

In order to run the program, some env variables needs to be set.

`CRDTL=ABCDEFGHIJ01234567` where the 10 first bytes are the sms system username and the 8 last bytes are the password.

`TARGETSYS=62.186.103.42` ip or domain of the targeted machine.

`FOLDER=/var/log/sms` target folder to send sms file to.

`MAILTO=addr@domain.com` mail target for mail alert.

Example:

```shell=
CRDTL=ABCDEFGHIJ01234567 TARGETSYS=62.186.103.42 FOLDER=/var/log/sms MAILTO="martin@gmail.com" ./randetect
```



---

## Build

Build a non dynamic binary. 2 architectures depending on model.


- aarch64 - Model DS418

```shell=
cross build --target=aarch64-unknown-linux-musl --release
```

- x86\_64  - Models RS814RP+, RS815RP+, RS818RP+...

```shell=
cross build --target=x86_64-unknown-linux-musl --release
```
