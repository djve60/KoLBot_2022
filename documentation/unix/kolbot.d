#! /bin/zsh

### BEGIN INIT INFO
# Provides:          KoLBot 
# Required-Start:    $local_fs $network
# Required-Stop:     $local_fs
# Default-Start:     2 3 4 5
# Default-Stop:      0 1 6
# Short-Description: KolBot service
# Description:       Run KolBot service
### END INIT INFO

SVCNAME="KoLBot"
SCRIPT=<COMMAND>
RUNAS=<USERNAME>

PIDFILE=/var/run/<SVCNAME>.pid
LOGFILE=/var/log/<SVCNAME>.log

start() {

    if [ -f /var/run/$PIDNAME ] && kill -0 $(cat /var/run/$PIDNAME); then
        echo '${SVCNAME} already running' >&2
        return 1
    fi

    echo 'Starting ${SVCNAME} …' >&2
    local CMD="$SCRIPT &> \"$LOGFILE\" & echo \$!"
    su -c "$CMD" $RUNAS > "$PIDFILE"
    echo '${SVCNAME} started' >&2
}

stop() {

    if [ ! -f "$PIDFILE" ] || ! kill -0 $(cat "$PIDFILE"); then
        echo '${SVCNAME} not running' >&2
        return 1
    fi

    echo 'Stopping ${SVCNAME}…' >&2
    kill -15 $(cat "$PIDFILE") && rm -f "$PIDFILE"
    echo '${SVCNAME} stopped' >&2
}

case "$1" in
  start)
    start
    ;;
  stop)
    stop
    ;;
  retart)
    stop
    start
    ;;
  *)
    echo "Usage: $0 {start|stop|restart}"
esac

exit 0
