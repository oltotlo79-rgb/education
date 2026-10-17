#!/usr/bin/env bash
# usage: serve.sh start <port> <dbfile> <logfile> [exe] | stop <port>
# Starts the bundled RustNote debug exe from rust10/target in the background.
here="$(cd "$(dirname "$0")" && pwd)"
case "$1" in
  start)
    PORT="$2"; DBFILE="$3"; LOG="$4"; EXE="${5:-$here/target/debug/rustnote.exe}"
    rm -f "$DBFILE" "$DBFILE-wal" "$DBFILE-shm"
    DATABASE_URL="$DBFILE" HOST=127.0.0.1 PORT="$PORT" "$EXE" > "$LOG" 2>&1 &
    for i in $(seq 1 100); do curl -s -o /dev/null "http://127.0.0.1:$PORT/health" && break; sleep 0.2; done
    curl -s "http://127.0.0.1:$PORT/health"; echo
    ;;
  stop)
    PORT="$2"
    for pid in $(netstat -ano | grep LISTENING | grep "127.0.0.1:$PORT " | awk '{print $5}' | sort -u); do
      taskkill //F //PID "$pid" > /dev/null 2>&1 && echo "stopped pid $pid"
    done
    ;;
esac
