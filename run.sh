#!/bin/bash

# Wifilog Background Process Management Script

# Define log and pid files
LOG_FILE="wifilog.log"
PID_FILE="wifilog.pid"

# Function to start the process
start_wifilog() {
    # Check if already running
    if [ -f "$PID_FILE" ] && kill -0 $(cat "$PID_FILE") 2>/dev/null; then
        echo "Wifilog is already running."
        exit 1
    fi

    # Build the project
    cargo build --release

    # Run the binary in background and redirect output
    nohup cargo run --release > "$LOG_FILE" 2>&1 & 

    # Capture the PID
    echo $! > "$PID_FILE"
    echo "Wifilog started in background. PID: $(cat "$PID_FILE")"
}

# Function to stop the process
stop_wifilog() {
    if [ -f "$PID_FILE" ]; then
        PID=$(cat "$PID_FILE")
        if kill -0 "$PID" 2>/dev/null; then
            kill "$PID"
            rm "$PID_FILE"
            echo "Wifilog stopped."
        else
            echo "Wifilog not running."
        fi
    else
        echo "No PID file found. Wifilog might not be running."
    fi
}

# Function to check status
status_wifilog() {
    if [ -f "$PID_FILE" ] && kill -0 $(cat "$PID_FILE") 2>/dev/null; then
        echo "Wifilog is running. PID: $(cat "$PID_FILE")"
        echo "Log file: $LOG_FILE"
    else
        echo "Wifilog is not running."
    fi
}

# Parse arguments
case "$1" in
    start)
        start_wifilog
        ;;
    stop)
        stop_wifilog
        ;;
    restart)
        stop_wifilog
        start_wifilog
        ;;
    status)
        status_wifilog
        ;;
    *)
        echo "Usage: $0 {start|stop|restart|status}"
        exit 1
esac

exit 0