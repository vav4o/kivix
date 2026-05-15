use std::io::Read;

pub fn run() {
    let current_branch = std::fs::read_to_string(".kiv/HEAD")
        .expect("Failed to read HEAD.")
        .trim()
        .to_string();

    let branch_name = current_branch.splitn(4, "/").last().unwrap_or("").to_string();
    println!("Current branch: {}", branch_name);

    
    let mut current_commit = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(&current_branch)
        .expect("Current branch does not exist.");

    let commit_hash = if current_commit.metadata().expect("file metadata not found").len() == 0 {
        None
    } else {
        let mut contents = String::new();
        current_commit.read_to_string(&mut contents)
            .expect("Failed to read file");

        Some(contents.trim().to_string())
    };
    
    if let Some(hash) = commit_hash {
        println!("Current commit: {}", hash);
    } else {
        println!("No commits yet.");
    }
}