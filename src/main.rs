use chrono::{DateTime, Local};
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use notify_rust::Notification;
use notify_rust::Timeout;
use std::thread;
use std::time::Duration;
use winapi::um::winuser::MessageBeep;

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
    let bar = ProgressBar::new(minutes * 60);
    bar.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} remaining: {eta}",
        )
        .unwrap()
        .progress_chars("##-"),
    );
    for _ in 0..(minutes * 60) {
        thread::sleep(Duration::from_secs(1));
        bar.inc(1)
    }

    // Get the current time in your local timezone
    let local_time: DateTime<Local> = Local::now();
    // Format the time as a string
    let time_string = local_time.format("%H:%M").to_string();

    // Display the notification
    let _ = Notification::new()
        .summary("Notification")
        .body(&format!(
            "{}: Time's up! Notification after {} minutes.",
            time_string, minutes
        ))
        .timeout(Timeout::Never) // this however is
        .show();

    // Play a sound
    unsafe {
        MessageBeep(0xFFFFFFFF);
    } // Plays the default system beep sound
}
