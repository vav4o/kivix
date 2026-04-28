use std::fs;

pub fn run() {
    println!("Initializing a new repository...");
    
    fs::create_dir(".kiv").unwrap();
    fs::create_dir(".kiv/objects").unwrap();
    fs::create_dir(".kiv/refs").unwrap();
    fs::create_dir(".kiv/refs/heads").unwrap();
    fs::write(".kiv/HEAD", "ref: refs/heads/main\n").unwrap();

    println!("Initialized kiv directory")
}