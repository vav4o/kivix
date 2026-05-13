use flate2::read::ZlibDecoder;
use std::io::Read;

//If reading a diff - applies them recursively up to the parent
pub fn read_hashed_file(file_hash: &str) -> String {
	let path = format!(".kiv/objects/{}/{}", &file_hash[..2], &file_hash[2..]);
	let compressed = std::fs::read(&path).expect("Failed to read object file");

	let mut decoder = ZlibDecoder::new(&compressed[..]);
	let mut decompressed: Vec<u8> = Vec::new();
	decoder.read_to_end(&mut decompressed).expect("Failed to decompress object");

	let header_pos = decompressed.iter().position(|&b| b == 0).expect("Invalid object: missing header separator");
	let header = std::str::from_utf8(&decompressed[..header_pos]).unwrap();
	let content = &decompressed[header_pos + 1..];

	if header.starts_with("diff ") {
		let mut parts = header.split_whitespace();
		parts.next(); // skip "diff"
		let parent_hash = parts.next().expect("Malformed diff header: missing parent hash");

		let parent_string = read_hashed_file(parent_hash);

		let diff_text = std::str::from_utf8(content).expect("Diff content is not valid UTF-8");

		return crate::commands::apply_diff::run_from_strings(diff_text, &parent_string);
	}

	let mut header_parts = header.split_whitespace();
	let obj_type = header_parts.next().unwrap_or("");

	match obj_type {
		"blob" | "commit"  => crate::tools::decoding::decode_content(content.to_vec()),

		"tree" => {
			let mut i: usize = 0;
			let mut out = String::new();

			while i < content.len() {
				let mode_start = i;
				while i < content.len() && content[i] != b' ' { i += 1; }
				if i >= content.len() { break; }
				let mode = std::str::from_utf8(&content[mode_start..i]).unwrap_or("");
				i += 1; // skip space

				let name_start = i;
				while i < content.len() && content[i] != 0 { i += 1; }
				if i >= content.len() { break; }
				let name = std::str::from_utf8(&content[name_start..i]).unwrap_or("");
				i += 1; // skip null

				if i + 20 > content.len() { break; }
				let hash_bytes = &content[i..i+20];
				let hash_hex = hex::encode(hash_bytes);
				i += 20;

				out.push_str(&format!("{}\t{}\t{}\n", mode, hash_hex, name));
			}

			out
		}

		_ => crate::tools::decoding::decode_content(content.to_vec()),
	}
}