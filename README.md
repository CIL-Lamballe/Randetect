# Synology NAS Ransomware Analyzer

## SQL queries

Non human activities detection or copy of files:

<img src="https://latex.codecogs.com/gif.latex?\small&space;\prod_{ip,&space;username}\sigma_{A.cmd\,=\,'create'&space;\;\cap\;&space;B.cmd\,=\,'write'&space;\;\cap\;&space;A.filename\,=\,B.filename&space;\;\cap\;&space;0\,\geq\,&space;B.time-A.time\,\leq\,&space;1}&space;\left&space;(&space;\rho_{A}&space;\left&space;(logs&space;\right&space;)&space;\times&space;\rho&space;B\left&space;(&space;logs&space;\right&space;)&space;\right&space;)" title="\small \prod_{ip, username}\sigma_{A.cmd\,=\,'create' \;\cap\; B.cmd\,=\,'write' \;\cap\; A.filename\,=\,B.filename \;\cap\; 0\,\geq\, B.time-A.time\,\leq\, 1} \left ( \rho_{A} \left (logs \right ) \times \rho_{B}\left ( logs \right ) \right )" />
