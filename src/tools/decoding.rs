pub fn decode_content(data: Vec<u8>) -> String {
    if data.is_empty() {
        return String::new();
    }
    
    // UTF-16LE BOM
    if data.len() >= 2 && data[0] == 0xFF && data[1] == 0xFE {
        let mut utf16_vec = Vec::new();
        for chunk in data[2..].chunks(2) {
            if chunk.len() == 2 {
                utf16_vec.push(u16::from_le_bytes([chunk[0], chunk[1]]));
            }
        }
        if let Ok(s) = String::from_utf16(&utf16_vec) {
            return s;
        }
    }
    
    // UTF-16LE
    if data.len() >= 4 {
        let mut ascii_null_pairs = 0;
        for chunk in data.chunks(2) {
            if chunk.len() == 2 && chunk[1] == 0x00 && (chunk[0] >= 0x20 && chunk[0] <= 0x7E || chunk[0] == b'\n' || chunk[0] == b'\r') {
                ascii_null_pairs += 1;
            }
        }
        if ascii_null_pairs as f64 / (data.len() as f64 / 2.0) > 0.7 {
            let mut utf16_vec = Vec::new();
            for chunk in data.chunks(2) {
                if chunk.len() == 2 {
                    utf16_vec.push(u16::from_le_bytes([chunk[0], chunk[1]]));
                }
            }
            if let Ok(s) = String::from_utf16(&utf16_vec) {
                return s;
            }
        }
    }
    
    // UTF-8
    if let Ok(s) = String::from_utf8(data.clone()) {
        return s;
    }
    
    let mut utf16_vec = Vec::new();
    for chunk in data.chunks(2) {
        if chunk.len() == 2 {
            utf16_vec.push(u16::from_le_bytes([chunk[0], chunk[1]]));
        }
    }
    String::from_utf16_lossy(&utf16_vec)
}