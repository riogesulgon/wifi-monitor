use std::process::Command;
use std::time::Duration;
use tokio::time;
use chrono::Local;

fn get_current_ssid() -> Option<String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg("networksetup -listallhardwareports | awk '/Wi-Fi|AirPort/{getline; print $NF}' | xargs networksetup -getairportnetwork | cut -d \" \" -f4")
        .output()
        .ok()?;
    
    if output.status.success() {
        let ssid = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !ssid.is_empty() {
            Some(ssid)
        } else {
            None
        }
    } else {
        None
    }
}

#[tokio::main]
async fn main() {
    let mut previous_ssid: Option<String> = None;
    let mut start_time =  Local::now();

    loop {
        match get_current_ssid() {
            Some(current_ssid) => {
                if previous_ssid.as_ref() != Some(&current_ssid) {
                    start_time = Local::now();
                    println!("[{}] Network changed: {}", start_time, current_ssid);
                    previous_ssid = Some(current_ssid);
                } else {
                    // calculate the time difference between the current and start time
                    let time_diff = Local::now().signed_duration_since(start_time);
                    // Subtract time_diff from 9 hours
                    let remaining = 60 * 60 * 9 - time_diff.num_seconds();
                    // Display the time remaining every hour
                    if time_diff.num_seconds() % 3600 == 0 {
                        println!("[{}] Time remaining: {} hour(s).", Local::now(), remaining);
                    }
                }
            },
            None => {
                let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                println!("[{}] Unable to retrieve network information", timestamp);
            }
        }

        // Wait for 5 seconds before checking again
        time::sleep(Duration::from_secs(5)).await;
    }
}
