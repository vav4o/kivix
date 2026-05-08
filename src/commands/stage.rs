pub fn run(){
    let stage_contents = std::fs::read_to_string("./.kiv/staging").unwrap_or_default();

    for line in stage_contents.lines() {
        let line = line.trim();
        if !line.is_empty() {
            if let Some(path) = line.split_whitespace().nth(1) {
                println!("{}", path);
            }
        }
    }
}