#!/bin/bash
TASKID=`synowebapi --exec name="custom" profile_applying=false api=SYNO.Core.Security.Firewall.Profile.Apply method=start version=1 | grep task_id | cut -d ':' -f2 | tr -d ' '`
sleep 0.5
synowebapi --exec api=SYNO.Core.Security.Firewall.Profile.Apply method=status version=1 task_id=$TASKID
sleep 0.5
synowebapi --exec api=SYNO.Core.Security.Firewall.Profile.Apply method=stop version=1
