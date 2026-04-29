use std::fs;

use hex;
//use clap::builder::Str;
//use std::{fs, os::unix::fs::PermissionsExt};
//use std::os::windows::fs::
//TODO: Rename hash_object for clarity
use crate::{commands::hash_object, tools::hash_object::hash_object};
use std::path::Path;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
pub fn run(write: bool) {
    println!("Writing tree...");

    let hash = hash_tree("./.kiv/staging", write);
    print!("Tree hash: {}", hash);
}

pub fn hash_tree(path: &str, write: bool) -> String {
    let mut dir = fs::read_dir(path).unwrap();
    let mut entries = Vec::new();
    while let Some(entry) = dir.next() {
        let entry = entry.unwrap();
        let name = entry.file_name();
        let meta = entry.metadata().unwrap();
        entries.push((entry, name, meta));
    }

    let mut tree = Vec::new();
    entries.sort_by(|a, b| a.1.cmp(&b.1));
    for (entry, name, meta) in entries {
        if name == ".kiv" {
            continue;
        }
        let mode = get_mode(&entry.path(), &meta);
        let hash = if meta.is_dir() {
            hash_tree(entry.path().to_str().unwrap(), write)
        } else {
            hash_object::hash_file(entry.path().to_str().unwrap(), write)
        };
        if hash.is_empty() {
            continue;
        }
        tree.extend(mode.as_bytes());
        tree.push(b' ');
        tree.extend(name.as_encoded_bytes());
        tree.push(b'\0');
        let decoded_hash = hex::decode(hash).unwrap();
        tree.extend(decoded_hash);
    }

    if tree.is_empty() {
        return "".to_string();
    } else {
        return hash_object(tree, "tree", write);
    }
}

fn get_mode(path: &Path, meta: &fs::Metadata) -> String {
    if meta.is_dir() {
        //directory
        return "40000".to_string();
    } else if meta.file_type().is_symlink() {
        //symlink
        return "120000".to_string();
    } else if is_executable(path, meta) {
        //executable
        return "100755".to_string();
    }
    //regular file
    "100644".to_string()
}

#[allow(unused)]
fn is_executable(path: &Path, meta: &fs::Metadata) -> bool {
    #[cfg(unix)]
    {
        return (meta.permissions().mode() & 0o111) != 0;
    }

    #[cfg(windows)]
    {
        return matches!(
            path.extension().and_then(|e| e.to_str()),
            Some("exe") | Some("bat") | Some("cmd")
        );
    }
}