use std::fs::File;
use std::io::Read;

use crate::tools::hash_object;

pub fn run(write: bool, file: String) {
    println!("Hashing object...");

    let hash = hash_file(&file, write);
    println!("Object hash: {}", hash);
}

pub fn hash_file(file_path: &str, write: bool) -> String {
    let mut file:File = File::open(file_path).expect("Failed to find/open provided file");
    let metadata = file.metadata().unwrap();

    if !metadata.is_file() {
        panic!("{} is not a file", file_path);
    }

    let mut buffer:Vec<u8> = vec![0; metadata.len() as usize];
    file.read(&mut buffer).unwrap();

    hash_object::hash_object(buffer, "blob", write)
}