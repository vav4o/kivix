pub fn run (branch_name: &str, commit_hash: Option<String>) {
    println!("Creating branch '{}'", branch_name);

    let branch_path = format!(".kiv/refs/branches/{}", branch_name);
    std::fs::write(
        &branch_path, 
        commit_hash.unwrap_or("".to_string())).expect("Failed to create branch");

    std::fs::write(".kiv/HEAD", format!(".kiv/refs/branches/{}\n", branch_name)).expect("Failed to update HEAD");
    
    println!("Branch '{}' created successfully.", branch_name);
}