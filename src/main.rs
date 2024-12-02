use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let target_commits = 1_000_000; // Set your target number of commits
    let mut commit_count = 0;

    println!("Starting commit generation...");

    for _ in 1..=target_commits {
        // Get the current time in milliseconds since UNIX epoch
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

      
        // Commit the changes with the timestamp as the commit message
        Command::new("git")
            .args(&["commit", "--allow-empty", "-m", &format!("Commit at {}", current_time)])
            .output()
            .expect("Failed to execute git commit");

        commit_count += 1;

        // Print progress every 1,000 commits
        if commit_count % 1000 == 0 {
            println!("{} commits created...", commit_count);
        }
    }

     // Step 2: Push the commits to the remote repository
    let push_output = Command::new("git")
        .args(&["push"])
        .output()
        .expect("Failed to execute git push");

        if !push_output.status.success() {
            eprintln!("Error pushing to remote repository: {}", String::from_utf8_lossy(&push_output.stderr));
        } else {
            println!("Successfully pushed {} commits to the remote repository!", commit_count);
        }
    
        println!("Finished! A total of {} commits have been made.", target_commits);
}
