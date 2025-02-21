
use rusqlite::{Connection, Result};
use std::process::Command;
use std::time::Duration;
use tokio::time;
use chrono::{Local, DateTime, NaiveDateTime, TimeZone};

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

// Initialize SQLite database and create table if not exists
fn init_db() -> Result<Connection> {
    let conn = Connection::open("network_times.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS network_change (
            id INTEGER PRIMARY KEY,
            start_time TEXT NOT NULL,
            network_name TEXT NOT NULL
        )",
        [],
    )?;
    Ok(conn)
}

/// Update or insert the start time in the database
fn update_start_time(conn: &Connection, start_time: &DateTime<Local>) -> Result<()> {
    let network_name = get_current_ssid().unwrap_or_else(|| "Unknown".to_string());
    conn.execute(
        "INSERT INTO network_change (start_time, network_name) VALUES (?1, ?2)",
        [
            start_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            network_name,
        ],
    )?;
    Ok(())
}

/// Get the earliest start time for today from the database
fn get_earliest_start_date(conn: &Connection) -> Result<DateTime<Local>> {
    let date = Local::now().format("%Y-%m-%d").to_string();
    let mut statement = conn.prepare("SELECT start_time FROM network_change \
        WHERE start_time LIKE ? ORDER BY id ASC LIMIT 1")?;
    let first_row = statement.query_row([date + "%"], |row| {
          Ok(row.get::<_, String>(0)?)
    });

    match first_row {
        Ok(date) => {
            let naive_date = NaiveDateTime::parse_from_str(&date, "%Y-%m-%d %H:%M:%S").unwrap();
            let local_date = Local.from_local_datetime(&naive_date).earliest().unwrap();
            Ok(local_date)
        },
        Err(e) => {
            println!("Error: {}", e);
            Err(e)
        }
    }
}

#[tokio::main]
async fn main() {
    let conn = init_db().expect("Failed to initialize database");
    let mut previous_ssid: Option<String> = None;

    loop {
        match get_current_ssid() {
            Some(current_ssid) => {
                if previous_ssid.as_ref() != Some(&current_ssid) {
                    let current_time = Local::now();
                    update_start_time(&conn, &current_time).expect("Failed to update start time");
                    println!("[{}] Network changed: {}", current_time, current_ssid);
                    previous_ssid = Some(current_ssid);
                } else {
                    let start_time = get_earliest_start_date(&conn).expect("Failed to retrieve start time");
                    println!("start_time: {}", start_time);
                    // calculate the elapsed time since the start_time
                    let elapsed = Local::now().signed_duration_since(start_time).num_seconds();
                    // convert the elapsed time to hours, minutes, and seconds
                    let hours = elapsed / 3600;
                    let minutes = (elapsed % 3600) / 60;
                    let seconds = elapsed % 60;
                    println!("Elapsed time: {} hours, {} minutes, {} seconds", hours, minutes, seconds);
                }
            },
            None => {
                let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                println!("[{}] Unable to retrieve network information", timestamp);
            }
        }

        time::sleep(Duration::from_secs(5)).await;
    }
}
