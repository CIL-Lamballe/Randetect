#!/bin/bash

echo $1

#Set IP
synowebapi --exec profile=\{\"global\":\{\"policy\":\"none\",\"rules\":\[\{\"enable\":true,\"name\":\"\",\"port_direction\":\"\",\"port_group\":\"all\",\"ports\":\"all\",\"protocol\":\"all\",\"source_ip_group\":\"ip\",\"source_ip\":\"$1\",\"policy\":\"drop\",\"log\":false\}\]\},\"name\":\"custom\"\} profile_applying=true api=SYNO.Core.Security.Firewall.Profile method=set version=1
sleep 0.5

# Apply Profile
TASKID=`synowebapi --exec name="custom" profile_applying=true api=SYNO.Core.Security.Firewall.Profile.Apply method=start version=1 | grep task_id | cut -d ':' -f2 | tr -d ' '`
sleep 0.5
synowebapi --exec task_id=$TASKID api=SYNO.Core.Security.Firewall.Profile.Apply method=status version=1
sleep 0.5

# Update settings
synowebapi --exec api=SYNO.Core.Security.Firewall.Profile.Apply method=stop version=1
sleep 0.5
/sbin/restart smbd
