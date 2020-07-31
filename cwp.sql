select A.ip, A.username, A.filename, B.filesize as wrotefilesize, A.cmd, A.time, B.cmd, B.time as btime
from logs A, logs B
where A.isdir=0 and B.isdir=0 and A.filename=B.filename and A.cmd='create' and B.cmd='write' and A.time<=B.time and (B.time-A.time)<=$xmin;
