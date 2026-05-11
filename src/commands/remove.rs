use std::fs;

use crate::tools::normalize_format::normalize_path;

pub fn run(file: String) {
    println!("Removing file: {} from staging", file);

    let staging_path = ".kiv/staging";
    let normalized_file = normalize_path(&file);

    let existing = fs::read_to_string(staging_path).unwrap_or_default();
    let mut new_lines: Vec<String> = Vec::new();
    let mut found = false;

    for line in existing.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split("   ").collect();
        match parts.as_slice() {
            [_status, _file_hash, file_name, _mtime, old_version_hash, original_mtime]
                if normalize_path(file_name) == normalized_file => {
                    found = true;

                    new_lines.push(format!(
                        "O   {}   {}   {}",
                        old_version_hash, file_name, original_mtime
                    ));
                    println!("reverted {} (old hash: {})", file, old_version_hash);
                }
            ["N", _file_hash, file_name, _mtime] 
                if normalize_path(file_name) == normalized_file => {
                    found = true;
                    
                    println!("removed {} from staging", file);
                }
            _ => new_lines.push(line.to_string()),
        }
    }

    if !found {
        println!("file {} not found in staging", file);
        return;
    }

    let mut content = new_lines.join("\n");
    if !content.is_empty() && !content.ends_with('\n') {
        content.push('\n');
    }
    fs::write(staging_path, content).expect("failed to write staging file");
}
