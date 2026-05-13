use chrono::{Local, TimeZone};

pub fn run(full: bool, time: bool){
    let stage_contents = std::fs::read_to_string("./.kiv/staging").unwrap_or_default();

    for line in stage_contents.lines() {
		if line.trim().is_empty() {
			continue;
		}

		let mut parts = line.splitn(6, "   ");
		let _ = parts
			.next()
			.expect("Malformed stage line: missing size");
		let status = parts
			.next()
			.expect("Malformed stage line: missing status");

        if !full && status == "O" {
            continue;
        }

		let _ = parts
			.next()
			.expect("Malformed stage line: missing hash");
		let path = parts
			.next()
			.expect("Malformed stage line: missing path");
		let normalized_path = path.strip_prefix(".\\").unwrap_or(path);

        if time {
            let time = parts
                .next()
                .expect("Malformed stage line: missing time");
            let formatted_time = format_stage_time(time);
        
		    println!("{:<2}   {:<25}   {}", status, normalized_path, formatted_time);
        } else {
            println!("{:<2}   {:<25}", status, normalized_path);		
        }
        
	}
}

fn format_stage_time(raw_time: &str) -> String {
	let Ok(ms_since_epoch) = raw_time.parse::<i64>() else {
		return raw_time.to_string();
	};

	let Some(dt) = Local.timestamp_millis_opt(ms_since_epoch).single() else {
		return raw_time.to_string();
	};

	dt.format("%H:%M %d:%m:%Y").to_string()
}