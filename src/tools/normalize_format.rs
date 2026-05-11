pub fn normalize_path(path: &str) -> String {
    let normalized = path.replace('\\', "/");
    if normalized.starts_with("./") {
        normalized[2..].to_string()
    } else {
        normalized
    }
}

pub fn normalize_line_endings(text: String) -> String {
    text.replace("\r\n", "\n").replace('\r', "\n")
}