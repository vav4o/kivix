use std::fs;

use crate::tools::normalize_path::normalize_path;

pub fn run(file: String) {
    println!("Deleting file: {}", file);

    let staging_path = ".kiv/staging";
    let normalized_file = normalize_path(&file);

    let existing = fs::read_to_string(staging_path).unwrap_or_default();
    let mut new_lines: Vec<String> = Vec::new();

    for line in existing.lines() {
        if line.trim().is_empty() {
            continue;
        }

        
        let parts: Vec<&str> = line.split("   ").collect();
        match parts.as_slice() {
            ["N", _, file_name, _] 
                if normalize_path(file_name) == normalized_file => {
                    
                    println!("removed {} from staging", file);
                }
            ["O", _, file_name, _]
                if normalize_path(file_name) == normalized_file => {
                    
                    println!("removed {} from branch", file);
                }
            ["M", _, file_name, _, _, _]
                if normalize_path(file_name) == normalized_file => {

                    println!("removed {} from branch", file);
                }
            _ => new_lines.push(line.to_string()),
        }
    }

    let mut content = new_lines.join("\n");
    if !content.is_empty() && !content.ends_with('\n') {
        content.push('\n');
    }
    fs::write(staging_path, content).expect("failed to write staging file");
}