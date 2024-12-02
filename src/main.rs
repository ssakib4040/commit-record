use std::process::Command;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use std::io::Write;

fn main() {
    let target_commits = 10_000_000; // Set your target number of commits
    let mut commit_count = 0;

    println!("Starting commit generation...");

    for _ in 1..=target_commits {
        // Get the current time in milliseconds since UNIX epoch
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

        // Append the timestamp to a text file
        let file_name = "timestamps.txt";
        let mut file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(file_name)
            .expect("Failed to open file");
        writeln!(file, "{}", current_time).expect("Failed to write to file");

        // Stage the changes
        Command::new("git")
            .args(&["add", file_name])
            .output()
            .expect("Failed to execute git add");

        // Commit the changes with the timestamp as the commit message
        Command::new("git")
            .args(&["commit", "-m", &format!("Commit at {}", current_time)])
            .output()
            .expect("Failed to execute git commit");

        commit_count += 1;

        // Print progress every 1,000 commits
        if commit_count % 1000 == 0 {
            println!("{} commits created...", commit_count);
        }
    }

    println!("Finished! A total of {} commits have been made.", target_commits);
}
