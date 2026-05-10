pub fn normalize_path(path: &str) -> String {
    let normalized = path.replace('\\', "/");
    if normalized.starts_with("./") {
        normalized[2..].to_string()
    } else {
        normalized
    }
}