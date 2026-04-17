
use crate::tools::hash_object::hash_object;

pub fn run(write: bool, tree_hash: String, parent_hash: Option<String>, message: String) {
    println!("Committing tree...");

    let hash = hash_commit(tree_hash, parent_hash.unwrap_or_else(|| "".into()), message, write);
    println!("Commit hash: {}", hash);
}

pub fn hash_commit(tree_sha: String, parent: String, message: String, write: bool) -> String {
    let parent: String = format!("parent {}", parent);
    let author: &str = "author My Name time +0000";

    let commit = format!(
        "tree {}\n{}\n{}\n\n{}\n",
        tree_sha, parent, author, message
    )
    .as_bytes()
    .to_vec();

    hash_object(commit, "commit", write)
}