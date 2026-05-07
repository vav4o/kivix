use std::fs;

pub fn run() {
    println!("Initializing a new repository...");
    
    fs::create_dir(".kiv").unwrap();
    fs::create_dir(".kiv/objects").unwrap();
    fs::create_dir(".kiv/refs").unwrap();
    fs::create_dir(".kiv/refs/branches").unwrap();
    fs::File::create("main").unwrap();
    fs::write(".kiv/HEAD", "refs/branches/main\n").unwrap();

    println!("Initialized kiv directory")
}