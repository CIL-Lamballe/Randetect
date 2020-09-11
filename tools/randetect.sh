#!/bin/sh -e
#
### BEGIN INIT INFO
# Provides:          Randetect
# Required-Start:    $all
# Required-Stop:
# Default-Start:     2 3 4 5
# Default-Stop:      0 1 6
# Short-Description: file audit detecting suspicious activity made from samba users.
### END INIT INFO

DAEMON="/bin/randetect"
#daemon_OPT="-d -p -f /home/user/randetect.ini"
DAEMONUSER="nobody"
daemon_NAME="randetect"

PATH="/sbin:/bin:/usr/sbin:/usr/bin"

test -x $DAEMON || exit 0

. /lib/lsb/init-functions

d_start () {
	log_daemon_msg "Starting system $daemon_NAME Daemon"
	start-stop-daemon --background --name $daemon_NAME --start --quiet --chuid $DAEMONUSER --exec $DAEMON #-- $daemon_OPT
	log_end_msg $?
}

d_stop () {
	log_daemon_msg "Stopping system $daemon_NAME Daemon"
	start-stop-daemon --name $daemon_NAME --stop --retry 5 --quiet --name $daemon_NAME
	log_end_msg $?
}

case "$1" in

	start|stop)
		d_${1}
		;;

	restart|reload|force-reload)
		d_stop
		d_start
		;;

	force-stop)
		d_stop
		killall -q $daemon_NAME || true
		sleep 2
		killall -q -9 $daemon_NAME || true
		;;

	status)
		status_of_proc "$daemon_NAME" "$DAEMON" "system-wide $daemon_NAME" && exit 0 || exit $?
		;;
	*)
		echo "Usage: /etc/init.d/$daemon_NAME {start|stop|force-stop|restart|reload|force-reload|status}"
		exit 1
		;;
esac
exit 0
