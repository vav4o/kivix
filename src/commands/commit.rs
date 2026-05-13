use std::io::{Read, Write};

use crate::commands::commit_tree;
use crate::tools::stage_to_tree;

pub fn run(message: Option<String>) {
    let message = message
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "No commit message".to_string());

    println!("Committing with message: {}", message);

    let branch = std::fs::read_to_string(".kiv/HEAD")
        .expect("Failed to read HEAD.")
        .trim()
        .to_string();
    let mut parent = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(&branch)
        .expect("Could not open file");

    let parent_hash = if parent.metadata().expect("file metadata not found").len() == 0 {
        None
    } else {
        let mut contents = String::new();
        parent.read_to_string(&mut contents)
            .expect("Failed to read file");

        Some(contents.trim().to_string())
    };

    let changes = stage_to_tree::create_tree("./.kiv/staging".to_string(), true);

    let new_commit = commit_tree::run(true, changes, parent_hash, message);

    parent.set_len(0).expect("Failed to truncate file!");
    parent.write_all(new_commit.as_bytes()).expect("Failed to write in parent");

    sync_staging_old_hashes("./.kiv/staging");
}

fn sync_staging_old_hashes(staging_path: &str) {
    let existing = std::fs::read_to_string(staging_path).unwrap_or_default();
    if existing.trim().is_empty() {
        return;
    }

    let mut new_lines: Vec<String> = Vec::new();

    for line in existing.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split("   ").collect();
        match parts.as_slice() {
            [accumulated_diff_size, _, file_hash, file_path, mtime, _, _, _] => {
                new_lines.push(format!("{}   O   {}   {}   {}", accumulated_diff_size, file_hash, file_path, mtime));
            }
            [accumulated_diff_size, _, file_hash, file_path, mtime] => {
                new_lines.push(format!("{}   O   {}   {}   {}", accumulated_diff_size, file_hash, file_path, mtime));
            }
            _ => {
                // Should be unreachable. Keeps the erroneous line
                new_lines.push(line.to_string());
            }
        }
    }

    let mut content = new_lines.join("\n");
    if !content.is_empty() {
        content.push('\n');
    }
    std::fs::write(staging_path, content).expect("Failed to write staging file");
}
