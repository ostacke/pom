extern crate notify_rust;
use notify_rust::Notification;
use std::thread;
use std::time::Duration;

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

    // Display the notification
    let _ = Notification::new()
        .summary("Notification")
        .body(&format!("Time's up! Notification after {} minutes.", minutes))
        .show();
}
