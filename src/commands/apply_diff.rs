

pub fn run(diff_file: String, target_file: String) -> String {
	let diff_content = std::fs::read_to_string(&diff_file)
		.expect("Failed to read diff file");
	let target_content = std::fs::read_to_string(&target_file)
		.expect("Failed to read target file");

	apply_diff_content(&diff_content, &target_content)
}

pub fn run_from_strings(diff_content: &str, target_content: &str) -> String {
	apply_diff_content(diff_content, target_content)
}

fn apply_diff_content(diff_content: &str, target_content: &str) -> String {
	let had_trailing_newline = target_content.ends_with('\n');
	let mut lines: Vec<String> = target_content
		.lines()
		.map(std::string::ToString::to_string)
		.collect();

	let mut offset: isize = 0;

	for raw_line in diff_content.lines() {
		if raw_line.trim().is_empty() {
			continue;
		}

		let mut parts = raw_line.splitn(4, ',');
		let line_number_str = parts
			.next()
			.expect("Malformed diff line: missing line number");
		let sign = parts
			.next()
			.expect("Malformed diff line: missing sign");
		let content1 = parts
			.next()
			.expect("Malformed diff line: missing content1");
		let content2 = parts.next();

		let line_number: usize = line_number_str.parse().unwrap();
		let sign_char = sign.chars().next().unwrap();

		let current_index = line_number as isize - 1 + offset;
		let index_with_offset = if current_index >= 0 {
			current_index as usize
		} else {
			0
		};
		let direct_index = line_number - 1;

		match sign_char {
			'+' => {
				let mut insertion_index = direct_index;
				if insertion_index > lines.len() {
					insertion_index = index_with_offset;
				}
				lines.insert(insertion_index, content1.to_string());
				offset += 1;
			}
			'-' => {
				let removal_index = resolve_exact_line_index(
					&lines,
					content1,
					direct_index,
					index_with_offset,
				);

				lines.remove(removal_index);
				offset -= 1;
			}
			'~' => {
				let replacement = content2.unwrap_or("");

				let replacement_index = resolve_exact_line_index(
					&lines,
					content1,
					direct_index,
					index_with_offset,
				);

				lines[replacement_index] = replacement.to_string();
			}
			_ => unreachable!(),
		}
	}

	let mut output = lines.join("\n");
	if had_trailing_newline {
		output.push('\n');
	}

	output
}

fn resolve_exact_line_index(
	lines: &[String],
	expected_content: &str,
	primary_index: usize,
	fallback_index: usize,
) -> usize {
	if primary_index < lines.len() && lines[primary_index] == expected_content {
		return primary_index;
	}

	if fallback_index < lines.len() && lines[fallback_index] == expected_content {
		return fallback_index;
	}

	panic!("Diff line allignment error!");
}