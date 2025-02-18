# WiFi Network Monitor 📡

A simple Rust application that monitors and logs WiFi network changes on macOS systems. 🖥️

## Features ✨

- Real-time WiFi network monitoring
- Timestamp-based logging
- Automatic detection of network changes
- Background process management
- Minimal resource usage

## Prerequisites 🔧

- macOS operating system
- Terminal access
- Internet connection (for installation)

## Installation 🚀

1. Install Rust (if you haven't already): 
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Clone this repository:
```
git clone https://github.com/riogesulgon/wifi-monitor.git
cd wifi-monitor
```

3. Make the run script executable:
```
chmod +x run.sh
```

## Usage 💻

The application can be managed using the `run.sh` script with the following commands:

### Start the monitor
```
./run.sh start
```

This will start the WiFi monitor in the background and create a log file (`wifilog.log`).

### Check status
```
./run.sh status
```

Shows if the monitor is running and displays the PID and log file location.

### Stop the monitor
```
./run.sh stop
```

### Restart the monitor
```
./run.sh restart
```

### View the logs 📊

You can view the logs in real-time using:
```
tail -f wifilog.log
```

The log will show entries like:
```
[2024-03-14 15:30:45] Network changed: MyWiFiNetwork
```

## How it Works 🔍

The application runs as a background process and checks your current WiFi network every 5 seconds using macOS system commands. When it detects a network change, it logs the new network name along with a timestamp to `wifilog.log`.

The `run.sh` script manages the process lifecycle and provides easy commands for starting, stopping, and monitoring the application.

## Contributing 🤝

Feel free to submit issues and enhancement requests!

## License 📝

MIT License