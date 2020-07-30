# Synology NAS Ransomware Detector

An implementation of a naive ransomware detection algorithm on Synology NAS.

This project aims at building a compiled object deamon for Synology NAS launched by contrab at boot time. It monitors the changes made to the filesystem by clients, filtering it with a customizable set of rules.

Encrypting does not inherently make a file larger but most of the time it does, for this reason we randomly checksum the files been raised by the alert.

## NAS Log Queries Relational Algebra

<img src="https://latex.codecogs.com/gif.latex?\small&space;\prod_{ip,&space;username}\sigma_{A.cmd\,=\,'create'&space;\;\cap\;&space;B.cmd\,=\,'write'&space;\;\cap\;&space;A.filename\,=\,B.filename&space;\;\cap\;&space;0\,\geq\,&space;B.time-A.time\,\leq\,&space;1}&space;\left&space;(&space;\rho_{A}&space;\left&space;(logs&space;\right&space;)&space;\times&space;\rho_{B}\left&space;(&space;logs&space;\right&space;)&space;\right&space;)" title="\small \prod_{ip, username}\sigma_{A.cmd\,=\,'create' \;\cap\; B.cmd\,=\,'write' \;\cap\; A.filename\,=\,B.filename \;\cap\; 0\,\geq\, B.time-A.time\,\leq\, 1} \left ( \rho_{A} \left (logs \right ) \times \rho_{B}\left ( logs \right ) \right )" />


<img src="https://latex.codecogs.com/gif.latex?\small&space;\prod_{ip,&space;username}\sigma_{C.cmd\,=\,'read'&space;\;\cap\;&space;D.cmd\,=\,'delete'&space;\;\cap\;&space;C.filename\,=\,D.filename&space;\;\cap\;&space;0\,\geq\,&space;D.time-C.time\,\leq\,&space;y,\:&space;y\,&space;\neq\,&space;0&space;}&space;\left&space;(&space;\rho_{C}&space;\left&space;(logs&space;\right&space;)&space;\times&space;\rho_{D}\left&space;(&space;logs&space;\right&space;)&space;\right&space;)" title="\small \prod_{ip, username}\sigma_{C.cmd\,=\,'read' \;\cap\; D.cmd\,=\,'delete' \;\cap\; C.filename\,=\,D.filename \;\cap\; 0\,\geq\, D.time-C.time\,\leq\, y,\: y\, \neq\, 0 } \left ( \rho_{C} \left (logs \right ) \times \rho_{D}\left ( logs \right ) \right )" />
