use std::io::{Read, Write};

use crate::commands::commit_tree;
use crate::tools::stage_to_tree;

pub fn run(message: String) {
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
        println!("Empty");
        None
    } else {
        println!("Wrong!");
        let mut contents = String::new();
        parent.read_to_string(&mut contents)
            .expect("Failed to read file");

        Some(contents.trim().to_string())
    };

    
    let changes = stage_to_tree::create_tree("./.kiv/staging".to_string(), true);
    //let changes = write_tree::hash_tree("./.kiv/staging", true);
    
    let new_commit = commit_tree::run(true, changes, parent_hash, message);

    parent.set_len(0).expect("Failed to truncate file!");
    parent.write_all(new_commit.as_bytes()).expect("Failed to write in parent");

    println!("New commit hash: {}", new_commit);
    
}