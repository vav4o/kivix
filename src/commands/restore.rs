pub fn run(file_hash: String, name: Option<String>) {
    println!("Restoring file with hash: {}", file_hash);

    let content = crate::tools::read_file::read_hashed_file(&file_hash);

    let filename = match name {
        Some(name) if !name.is_empty() => name,
        _ => file_hash.clone(),
    };

    std::fs::write(&filename, content).expect("Failed to write restored file");
    println!("File restored as '{}'", filename);
}