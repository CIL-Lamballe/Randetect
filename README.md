# Ransomware Detector for Synology NAS

An implementation of a naive ransomware detection algorithm running on Synology Network Attached Storage.

This project aims at building a deamon started at DSM boot time. The deamon will monitors the changes made to the filesystem by its clients, filtering it with a customizable set of rules and banning ips or powering off the NAS depending on criticity level.


## NAS Log Queries Relational Algebra

SQL Query from Synology Database details
Pairs read/delete within y seconds.
ymin: Time between a read and a delete operation on a same file

<img src="https://latex.codecogs.com/gif.latex?\small&space;\prod_{ip,&space;username}\sigma_{A.cmd\,=\,'create'&space;\;\cap\;&space;B.cmd\,=\,'write'&space;\;\cap\;&space;A.filename\,=\,B.filename&space;\;\cap\;&space;0\,\geq\,&space;B.time-A.time\,\leq\,&space;1}&space;\left&space;(&space;\rho_{A}&space;\left&space;(logs&space;\right&space;)&space;\times&space;\rho_{B}\left&space;(&space;logs&space;\right&space;)&space;\right&space;)" title="\small \prod_{ip, username}\sigma_{A.cmd\,=\,'create' \;\cap\; B.cmd\,=\,'write' \;\cap\; A.filename\,=\,B.filename \;\cap\; 0\,\geq\, B.time-A.time\,\leq\, 1} \left ( \rho_{A} \left (logs \right ) \times \rho_{B}\left ( logs \right ) \right )" />

Pairs create and write within x seconds.
xmin: Time between a create and a write operation on a same file.

<img src="https://latex.codecogs.com/gif.latex?\small&space;\prod_{ip,&space;username}\sigma_{C.cmd\,=\,'read'&space;\;\cap\;&space;D.cmd\,=\,'delete'&space;\;\cap\;&space;C.filename\,=\,D.filename&space;\;\cap\;&space;0\,\geq\,&space;D.time-C.time\,\leq\,&space;y,\:&space;y\,&space;\neq\,&space;0&space;}&space;\left&space;(&space;\rho_{C}&space;\left&space;(logs&space;\right&space;)&space;\times&space;\rho_{D}\left&space;(&space;logs&space;\right&space;)&space;\right&space;)" title="\small \prod_{ip, username}\sigma_{C.cmd\,=\,'read' \;\cap\; D.cmd\,=\,'delete' \;\cap\; C.filename\,=\,D.filename \;\cap\; 0\,\geq\, D.time-C.time\,\leq\, y,\: y\, \neq\, 0 } \left ( \rho_{C} \left (logs \right ) \times \rho_{D}\left ( logs \right ) \right )" />

Encrypting does not inherently make a file larger but most of the time it does, for this reason we randomly checksum the files been raised by the alert.
