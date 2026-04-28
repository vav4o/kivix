use crate::commands::hash_object;


pub fn run(file: String) {
    println!("Adding file: {}", file);

    let hash = hash_object::hash_file(&file, true, true);
    println!("File hash: {}", hash);


}