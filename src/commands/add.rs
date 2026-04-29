use std::{fs::OpenOptions, io::Write};

use crate::commands::hash_object;


pub fn run(file: String) {
    println!("Adding file: {}", file);

    let staging_path = ".kiv/staging";

    let hash = hash_object::hash_file(&file, true);

    let existing = std::fs::read_to_string(staging_path).unwrap_or_default();

    let mut found = false;
    let mut changed = false;
    let mut new_lines: Vec<String> = Vec::new();

    for line in existing.lines() {
        let mut parts = line.splitn(2, "   ");
        match (parts.next(), parts.next()) {
            (Some(old_hash), Some(file_name)) 
                if file_name == file => {
                    found = true;
                    if old_hash != hash {
                        new_lines.push(format!("{}   {}", hash, file));
                        changed = true;
                    } else {
                        new_lines.push(line.to_string());
                    }
                }
            _ => new_lines.push(line.to_string()),
        }
    }

    if !found {
        let mut staging = OpenOptions::new()
            .append(true)
            .create(true)
            .open(staging_path)
            .expect("failed to open staging file");
        writeln!(staging, "{}   {}", hash, file).expect("failed to write to staging");
        println!("{} {}", hash, file);
        return;
    }

    if changed {
        let mut content = new_lines.join("\n");
        if !content.ends_with('\n') {
            content.push('\n');
        }
        std::fs::write(staging_path, content).expect("failed to write staging file");
        println!("updated {} {}", hash, file);
    } else {
        println!("unchanged {} {}", hash, file);
    }
}