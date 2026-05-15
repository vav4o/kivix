pub fn run (commit_hash: &str) {
    let head_path = ".kiv/HEAD";
    let head_content = std::fs::read_to_string(head_path).expect("Failed to read HEAD");
    let current_branch_path = head_content.trim();
    
    std::fs::write(current_branch_path, commit_hash).expect("Failed to update current branch with commit hash");
    
    println!("Current branch updated with commit hash '{}'.", commit_hash);
}