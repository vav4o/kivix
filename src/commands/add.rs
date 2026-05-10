use std::{
    fs::OpenOptions,
    io::Write,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::commands::hash_object;
use crate::tools::normalize_path::normalize_path;

//If file name doesn't exist: add it (3 fields)
//If file name exists: check last modified time. If not matching -> calculate and check hash, if no match -> update (5 fields)
pub fn run(file: String) {
    println!("Adding file: {}", file);

    let staging_path = ".kiv/staging";
    let normalized_file = normalize_path(&file);
    let current_mtime = get_modified_time(&file);
    let current_mtime_str = current_mtime.to_string();

    let existing = std::fs::read_to_string(staging_path).unwrap_or_default();

    let mut found = false;
    let mut changed = false;
    let mut new_lines: Vec<String> = Vec::new();
    let mut computed_hash: Option<String> = None;

    for line in existing.lines() {
        let parts: Vec<&str> = line.split("   ").collect();
        match parts.as_slice() {
            [_status, old_hash, file_name, recorded_mtime, maybe_old_version_hash, maybe_original_mtime]
                if normalize_path(file_name) == normalized_file => {
                found = true;
                if *recorded_mtime == current_mtime_str {
                    new_lines.push(line.to_string());
                } else {
                    let hash = computed_hash
                        .get_or_insert_with(|| hash_object::hash_file(&file, true))
                        .clone();
                    if hash == *old_hash {
                        new_lines.push(line.to_string());
                    } else {
                        new_lines.push(format!(
                            "M   {}   {}   {}   {}   {}",
                            hash, normalized_file, current_mtime_str, maybe_old_version_hash, maybe_original_mtime
                        ));
                        changed = true;
                    }
                }
            }
            [status, old_hash, file_name, recorded_mtime]
                if normalize_path(file_name) == normalized_file => {
                found = true;
                if *recorded_mtime == current_mtime_str {
                    new_lines.push(line.to_string());
                } else {
                    let hash = computed_hash
                        .get_or_insert_with(|| hash_object::hash_file(&file, true))
                        .clone();
                    if hash == *old_hash {
                        new_lines.push(line.to_string());
                    } else {
                        if status == &"N" {
                            new_lines.push(format!(
                                "N   {}   {}   {}",
                                hash, normalized_file, current_mtime_str
                            ));
                        } else {
                            new_lines.push(format!(
                                "M   {}   {}   {}   {}   {}",
                                hash, normalized_file, current_mtime_str, old_hash, recorded_mtime
                            ));
                        }
                        changed = true;
                    }
                }
            }
            _ => new_lines.push(line.to_string()),
        }
    }

    if !found {
        let hash = computed_hash
            .get_or_insert_with(|| hash_object::hash_file(&file, true))
            .clone();
        let mut staging = OpenOptions::new()
            .append(true)
            .create(true)
            .open(staging_path)
            .expect("failed to open staging file");
        
        writeln!(staging, "N   {}   {}   {}", hash, normalized_file, current_mtime_str)
            .expect("failed to write to staging");
        println!("{} {} {}", hash, normalized_file, current_mtime_str);
        return;
    }

    if changed {
        let mut content = new_lines.join("\n");
        if !content.ends_with('\n') {
            content.push('\n');
        }
        std::fs::write(staging_path, content).expect("failed to write staging file");
        let hash = computed_hash.unwrap_or_default();
        println!("updated {} {} {}", hash, file, current_mtime_str);
    } else {
        println!("unchanged {} {}", file, current_mtime_str);
    }
}

fn get_modified_time(path: &str) -> u128 {
    let metadata = std::fs::metadata(path).expect("failed to read file metadata");
    let modified: SystemTime = metadata.modified().expect("failed to read modified time");
    modified
        .duration_since(UNIX_EPOCH)
        .expect("modified time is before UNIX_EPOCH")
        .as_millis()
}
