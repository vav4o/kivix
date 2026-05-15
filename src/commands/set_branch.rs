pub fn run (branch_name: &str) {
    let branch_path = format!(".kiv/refs/branches/{}", branch_name);
    if !std::path::Path::new(&branch_path).exists() {
        eprintln!("Branch '{}' does not exist.", branch_name);
        return;
    }

    std::fs::write(".kiv/HEAD", format!(".kiv/refs/branches/{}\n", branch_name)).expect("Failed to update HEAD");
    
    println!("Current branch set to '{}'.", branch_name);
}