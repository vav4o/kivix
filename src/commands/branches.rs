use std::fs;

pub fn run(commits: bool) {
	let branches_dir = ".kiv/refs/branches";

	let mut branch_entries = fs::read_dir(branches_dir)
		.expect("Failed to read branches directory.")
		.filter_map(|entry| entry.ok())
		.collect::<Vec<_>>();

	branch_entries.sort_by_key(|entry| entry.file_name());

	for entry in branch_entries {
		let branch_name = entry.file_name().to_string_lossy().into_owned();

        let mut commit_hash = "".to_string();
		if commits == true {
			commit_hash = fs::read_to_string(entry.path())
				.unwrap_or_default()
				.trim()
				.to_string();
			
		}
        println!("{:<25} {}", branch_name, commit_hash);
	}

	
}