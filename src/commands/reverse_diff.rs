pub fn run(diff_file: String) {
    println!("Running reverse diff command...");

    let diff = std::fs::read_to_string(&diff_file).expect("Failed to read diff file");

    let mut new_diff: Vec<String> = Vec::new();

    for line in diff.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(",").collect();
        match parts.as_slice() {
            [line_number, "+", content1] => {
                new_diff.push(format!("{},-,{}", line_number, content1));
            }
            [line_number, "-", content1] => {
                new_diff.push(format!("{},+,{}", line_number, content1));
            }
            [line_number, "~", content1, content2] => {
                new_diff.push(format!("{},~,{},{}",line_number, content2, content1));
            }
            _ => {
                println!("Unrecognized diff line format: {}", line);
                return;
            }
        }
    }

    println!("Diff:\n{}", String::from_utf8_lossy(&new_diff.join("\n").as_bytes()));
}