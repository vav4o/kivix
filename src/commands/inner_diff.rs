use imara_diff::{Algorithm, Diff, InternedInput};

pub fn run(file1: String, file2: String) {
    println!("Running diff command...");

    let rendered = inner_diff(file1, file2);

    println!("Diff:\n{}", rendered);
}
//This method brute-forces the output from a Myers diff to produce the smallest possible 
//diff in order to optimize memory storage and reverse-diffing. 
//Do not display it as it is not readable and also wrong(when displaying replacements)!
//TODO: Move this to tools
fn inner_diff(file1: String, file2: String) -> String {
    let before = std::fs::read_to_string(&file1).expect("Failed to read before.txt");
    let after = std::fs::read_to_string(&file2).expect("Failed to read after.txt");

    let input = InternedInput::new(before.as_str(), after.as_str());
    let mut diff = Diff::compute(Algorithm::Myers, &input);
    diff.postprocess_lines(&input);

    let before_lines: Vec<&str> = before.lines().collect();
    let after_lines: Vec<&str> = after.lines().collect();
    let mut rendered = String::new();

    for hunk in diff.hunks() {
        let before_start = hunk.before.start as usize;
        let before_end = hunk.before.end as usize;
        let after_start = hunk.after.start as usize;
        let after_end = hunk.after.end as usize;

        let before_len = before_end.saturating_sub(before_start);
        let after_len = after_end.saturating_sub(after_start);
        let replaced_len = before_len.min(after_len);

        for offset in 0..replaced_len {
            let line_number = after_start + offset + 1;
            let old_line = before_lines[before_start + offset];
            let new_line = after_lines[after_start + offset];
            rendered.push_str(&format!("{}, ~, {}, {}\n", line_number, old_line, new_line));
        }

        for line_index in before_start + replaced_len..before_end {
            rendered.push_str(&format!("{}, -, {}\n", line_index + 1, before_lines[line_index]));
        }

        for line_index in after_start + replaced_len..after_end {
            rendered.push_str(&format!("{}, +, {}\n", line_index + 1, after_lines[line_index]));
        }
    }

    rendered
}
