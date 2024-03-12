use notify_rust::Notification;
use std::thread;
use std::time::Duration;
use notify_rust::Timeout;
use winapi::um::winuser::MessageBeep;
use chrono::{Local, DateTime};

fn main() {
    // Get the number of minutes to wait from command line arguments
    let args: Vec<String> = std::env::args().collect();
    let minutes = match args.get(1) {
        Some(arg) => match arg.parse::<u64>() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please provide a valid number of minutes.");
                return;
            }
        },
        None => {
            println!("Usage: pom <number of minutes>");
            return;
        }
    };

    // Wait for the specified number of seconds
    println!("Waiting for {} minutes...", minutes);
    thread::sleep(Duration::from_secs(minutes * 60));
    
    // Get the current time in your local timezone
    let local_time: DateTime<Local> = Local::now();
    // Format the time as a string
    let time_string = local_time.format("%H:%M").to_string();

    // Display the notification
    let _ = Notification::new()
        .summary("Notification")
        .body(&format!("{}: Time's up! Notification after {} minutes.", time_string, minutes))
        .timeout(Timeout::Never) // this however is
        .show();

    // Play a sound
    unsafe { MessageBeep(0xFFFFFFFF); } // Plays the default system beep sound
}
