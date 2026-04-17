use flate2::read::ZlibDecoder;



pub fn run(pretty_print: bool, object_hash: String) {
    //println!("Displaying file contents...");

    let compressed_data = std::fs::read(
        format!(".kiv/objects/{}/{}", &object_hash[..2], &object_hash[2..]))
        .expect("Failed to read object file");


    let mut decoder = ZlibDecoder::new(&compressed_data[..]);
    let mut decompressed_data = Vec::new();
    std::io::copy(&mut decoder, &mut decompressed_data).unwrap();

    if pretty_print {
        // Skip the header (type and size) and null byte
        let content_start = decompressed_data.iter().position(|&b| b == 0).unwrap() + 1;
        let content = &decompressed_data[content_start..];
        println!("{}", String::from_utf8_lossy(content));
    } else {
        println!("{}", String::from_utf8_lossy(&decompressed_data));
    }
}