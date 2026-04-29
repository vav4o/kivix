use imara_diff::{Algorithm, Diff, InternedInput};
use crate::tools::hash_object;
use std::io::Write;

pub fn run(file1: String, file2: String) {
    println!("Running diff command...");

    let rendered = inner_diff(file1, file2);

    let hash = hash_object::hash_object(rendered.clone(), "diff", true);

    println!("Diff:\n{}", String::from_utf8_lossy(&rendered));
    
    println!("Hash: {}", hash);

    //Under work
    let attr = std::fs::metadata(format!(".kiv/objects/{}/{}", &hash[..2], &hash[2..]))
        .expect("Failed to get file metadata").len();

    println!("File size: {} bytes", attr);
    //
}
//This method brute-forces the output from a Myers diff to produce the smallest possible 
//diff in order to optimize memory storage and reverse-diffing. 
//Do not display it as it is not readable and also wrong(when displaying replacements)!
//TODO: Move this to tools
fn inner_diff(file1: String, file2: String) -> Vec<u8> {
    let before = std::fs::read_to_string(&file1).expect("Failed to read before.txt");
    let after = std::fs::read_to_string(&file2).expect("Failed to read after.txt");

    let input = InternedInput::new(before.as_str(), after.as_str());
    let mut diff = Diff::compute(Algorithm::Myers, &input);
    diff.postprocess_lines(&input);

    let before_lines: Vec<&str> = before.lines().collect();
    let after_lines: Vec<&str> = after.lines().collect();
    let mut rendered: Vec<u8> = Vec::new();

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
            writeln!(&mut rendered, "{},~,{},{}", line_number, old_line, new_line)
                .expect("writing to buffer failed");
        }

        for line_index in before_start + replaced_len..before_end {
            writeln!(&mut rendered, "{},-,{}", line_index + 1, before_lines[line_index])
                .expect("writing to buffer failed");
        }

        for line_index in after_start + replaced_len..after_end {
            writeln!(&mut rendered, "{},+,{}", line_index + 1, after_lines[line_index])
                .expect("writing to buffer failed");
        }
    }

    rendered
}


