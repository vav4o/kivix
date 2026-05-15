use imara_diff::{Algorithm, Diff, InternedInput};

use crate::tools::normalize_format::normalize_line_endings;

#[derive(Clone, Debug)]
struct Edit {
	start: usize,
	end: usize,
	replacement: Vec<String>,
}

pub fn three_way_merge(base_file: String, my_file: String, their_file: String) -> String {
	let base_content = read_normalized_file(&base_file);
	let my_content = read_normalized_file(&my_file);
	let their_content = read_normalized_file(&their_file);

	let had_trailing_newline = base_content.ends_with('\n')
		|| my_content.ends_with('\n')
		|| their_content.ends_with('\n');

	let base_lines: Vec<&str> = base_content.lines().collect();
	let my_lines: Vec<&str> = my_content.lines().collect();
	let their_lines: Vec<&str> = their_content.lines().collect();

	let my_edits = compute_edits(&base_content, &my_content, &my_lines);
	let their_edits = compute_edits(&base_content, &their_content, &their_lines);
	let merged_lines = merge_edits(&base_lines, &my_edits, &their_edits);

	let mut merged_content = merged_lines.join("\n");
	if had_trailing_newline {
		merged_content.push('\n');
	}

	merged_content
}

//Was skipping edits when they were insertions at the end of the file, 
//should be fine now, but if it breaks check around error message "checked above"
fn merge_edits(base_lines: &[&str], my_edits: &[Edit], their_edits: &[Edit]) -> Vec<String> {
	let mut merged_lines: Vec<String> = Vec::new();
	let mut base_pos = 0;
	let mut my_index = 0;
	let mut their_index = 0;

	loop {
		while my_index < my_edits.len() && my_edits[my_index].end < base_pos {
			my_index += 1;
		}
		while their_index < their_edits.len() && their_edits[their_index].end < base_pos {
			their_index += 1;
		}

		let my_edit = my_edits.get(my_index).filter(|edit| edit.start == base_pos);
		let their_edit = their_edits.get(their_index).filter(|edit| edit.start == base_pos);

		if my_edit.is_none() && their_edit.is_none() {
			if base_pos >= base_lines.len() {
				break;
			}

			merged_lines.push(base_lines[base_pos].to_string());
			base_pos += 1;
			continue;
		}

		if let Some(edit) = my_edit {
			if edit.start == edit.end {
				push_inserted_lines(&mut merged_lines, &edit.replacement);
				my_index += 1;
			}
		}

		if let Some(edit) = their_edit {
			if edit.start == edit.end {
				if my_edit.is_none() {
					push_inserted_lines(&mut merged_lines, &edit.replacement);
					their_index += 1;
				} else if my_edit.map(|mine| mine.start == mine.end).unwrap_or(false) {
					let mine = my_edit.expect("checked above");
					if mine.replacement == edit.replacement {
						push_inserted_lines(&mut merged_lines, &mine.replacement);
					} else {
						push_conflict(
							&mut merged_lines,
							mine.replacement.as_slice(),
							edit.replacement.as_slice(),
						);
					}
					their_index += 1;
				} else {
					push_inserted_lines(&mut merged_lines, &edit.replacement);
					their_index += 1;
				}
			}
		}

		let my_edit = my_edits.get(my_index).filter(|edit| edit.start == base_pos && edit.end > base_pos);
		let their_edit = their_edits.get(their_index).filter(|edit| edit.start == base_pos && edit.end > base_pos);

		match (my_edit, their_edit) {
			(Some(mine), Some(theirs)) => {
				if mine.end == theirs.end && mine.replacement == theirs.replacement {
					merged_lines.extend(mine.replacement.iter().cloned());
				} else {
					push_conflict(
						&mut merged_lines,
						mine.replacement.as_slice(),
						theirs.replacement.as_slice(),
					);
				}

				base_pos = mine.end.max(theirs.end);
				my_index += 1;
				their_index += 1;
			}
			(Some(mine), None) => {
				if let Some(theirs) = their_edits
					.get(their_index)
					.filter(|edit| edit.start < mine.end && edit.end > base_pos)
				{
					let overlap_end = mine.end.max(theirs.end);

					let mine_variant = mine.replacement.clone();
					let mut theirs_variant: Vec<String> = Vec::new();

					if base_pos < theirs.start {
						theirs_variant.extend(
							base_lines[base_pos..theirs.start]
								.iter()
								.map(|line| (*line).to_string()),
						);
					}

					theirs_variant.extend(theirs.replacement.iter().cloned());

					if theirs.end < overlap_end {
						theirs_variant.extend(
							base_lines[theirs.end..overlap_end]
								.iter()
								.map(|line| (*line).to_string()),
						);
					}

					push_conflict(
						&mut merged_lines,
						mine_variant.as_slice(),
						theirs_variant.as_slice(),
					);

					base_pos = overlap_end;
					my_index += 1;
					their_index += 1;
				} else {
					merged_lines.extend(mine.replacement.iter().cloned());
					base_pos = mine.end;
					my_index += 1;
				}
			}
			(None, Some(theirs)) => {
				if let Some(mine) = my_edits
					.get(my_index)
					.filter(|edit| edit.start < theirs.end && edit.end > base_pos)
				{
					let overlap_end = mine.end.max(theirs.end);

					let mut mine_variant: Vec<String> = Vec::new();
					if base_pos < mine.start {
						mine_variant.extend(
							base_lines[base_pos..mine.start]
								.iter()
								.map(|line| (*line).to_string()),
						);
					}

					mine_variant.extend(mine.replacement.iter().cloned());

					if mine.end < overlap_end {
						mine_variant.extend(
							base_lines[mine.end..overlap_end]
								.iter()
								.map(|line| (*line).to_string()),
						);
					}

					let theirs_variant = theirs.replacement.clone();

					push_conflict(
						&mut merged_lines,
						mine_variant.as_slice(),
						theirs_variant.as_slice(),
					);

					base_pos = overlap_end;
					my_index += 1;
					their_index += 1;
				} else {
					merged_lines.extend(theirs.replacement.iter().cloned());
					base_pos = theirs.end;
					their_index += 1;
				}
			}
			(None, None) => {}
		}
	}

	merged_lines
}

fn push_inserted_lines(merged_lines: &mut Vec<String>, inserted_lines: &[String]) {
	merged_lines.extend(inserted_lines.iter().cloned());
}

fn push_conflict(merged_lines: &mut Vec<String>, mine: &[String], theirs: &[String]) {
	merged_lines.push("<<<<<<< mine".to_string());
	merged_lines.extend(mine.iter().cloned());
	merged_lines.push("=======".to_string());
	merged_lines.extend(theirs.iter().cloned());
	merged_lines.push(">>>>>>> theirs".to_string());
}

fn compute_edits(base_content: &str, other_content: &str, other_lines: &[&str]) -> Vec<Edit> {
	let input = InternedInput::new(base_content, other_content);
	let mut diff = Diff::compute(Algorithm::Myers, &input);
	diff.postprocess_lines(&input);

	let mut edits = Vec::new();

	for hunk in diff.hunks() {
		let replacement = other_lines[hunk.after.start as usize..hunk.after.end as usize]
			.iter()
			.map(|line| (*line).to_string())
			.collect();

		edits.push(Edit {
			start: hunk.before.start as usize,
			end: hunk.before.end as usize,
			replacement,
		});
	}

	edits
}

fn read_normalized_file(file_path: &str) -> String {
	let content = std::fs::read_to_string(file_path)
		.unwrap_or_else(|error| panic!("Failed to read {}: {}", file_path, error));
	normalize_line_endings(content)
}