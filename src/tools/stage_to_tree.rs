use std::collections::BTreeMap;

use crate::tools::hash_object;

#[derive(Default)]
struct TreeNode {
    entries: BTreeMap<String, TreeEntry>,
}

enum TreeEntry {
    Blob(String),
    Tree(TreeNode),
}

pub fn create_tree(file_path: String, write: bool) -> String {
    let stage_contents = std::fs::read_to_string(file_path).unwrap_or_default();
    let mut root = TreeNode::default();

    for line in stage_contents.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let mut parts = line.splitn(3, "   ");
        let hash = match parts.next() {
            Some(value) if !value.is_empty() => value,
            _ => continue,
        };
        let path = match parts.next() {
            Some(value) => value.trim(),
            None => continue,
        };

        if path.is_empty() {
            continue;
        }

        let normalized_path = path.replace('\\', "/");
        let components: Vec<&str> = normalized_path
            .split('/')
            .filter(|component| !component.is_empty() && *component != ".")
            .collect();

        if components.is_empty() {
            continue;
        }

        insert_entry(&mut root, &components, hash.to_string());
    }

    let tree_hash = build_tree_hash(&root, write).unwrap_or_default();
    println!("{}", tree_hash);
    tree_hash
}

fn insert_entry(node: &mut TreeNode, components: &[&str], blob_hash: String) {
    if components.len() == 1 {
        node.entries
            .insert(components[0].to_string(), TreeEntry::Blob(blob_hash));
        return;
    }

    let head = components[0].to_string();
    let entry = node
        .entries
        .entry(head)
        .or_insert_with(|| TreeEntry::Tree(TreeNode::default()));

    match entry {
        TreeEntry::Tree(child) => insert_entry(child, &components[1..], blob_hash),
        TreeEntry::Blob(_) => {
            *entry = TreeEntry::Tree(TreeNode::default());
            if let TreeEntry::Tree(child) = entry {
                insert_entry(child, &components[1..], blob_hash);
            }
        }
    }
}

fn build_tree_hash(node: &TreeNode, write: bool) -> Option<String> {
    let mut tree: Vec<u8> = Vec::new();

    for (name, entry) in &node.entries {
        let (mode, hash) = match entry {
            TreeEntry::Blob(hash) => ("100644", hash.clone()),
            TreeEntry::Tree(child) => match build_tree_hash(child, write) {
                Some(hash) => ("40000", hash),
                None => continue,
            },
        };

        tree.extend(mode.as_bytes());
        tree.push(b' ');
        tree.extend(name.as_bytes());
        tree.push(b'\0');
        tree.extend(hex::decode(hash).unwrap());
    }

    if tree.is_empty() {
        None
    } else {
        Some(hash_object::hash_object(tree, "tree", write))
    }
}