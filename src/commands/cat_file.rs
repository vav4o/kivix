use flate2::read::ZlibDecoder;

const TREE_PREFIX: &[u8] = b"tree ";



pub fn run(_pretty_print: bool, object_hash: String) {
    //println!("Displaying file contents...");

    let compressed_data = std::fs::read(
        format!(".kiv/objects/{}/{}", &object_hash[..2], &object_hash[2..]))
        .expect("Failed to read object file");


    let mut decoder = ZlibDecoder::new(&compressed_data[..]);
    let mut decompressed_data = Vec::new();
    std::io::copy(&mut decoder, &mut decompressed_data).unwrap();

    print_object(&decompressed_data);
}

fn print_object(data: &[u8]) {
    let content_start = data.iter().position(|&b| b == 0).unwrap() + 1;
    let header = std::str::from_utf8(&data[..content_start - 1]).unwrap();
    let content = &data[content_start..];

    if header.starts_with(std::str::from_utf8(TREE_PREFIX).unwrap()) {
        print_tree(content);
    } else {
        println!("{}", String::from_utf8_lossy(content));
    }
}

fn print_tree(content: &[u8]) {
    let mut index = 0;

    while index < content.len() {
        let mode_end = content[index..]
            .iter()
            .position(|&byte| byte == b' ')
            .expect("Invalid tree object: missing mode separator")
            + index;
        let mode = std::str::from_utf8(&content[index..mode_end]).unwrap();
        index = mode_end + 1;

        let name_end = content[index..]
            .iter()
            .position(|&byte| byte == 0)
            .expect("Invalid tree object: missing name terminator")
            + index;
        let name = std::str::from_utf8(&content[index..name_end]).unwrap();
        index = name_end + 1;

        let hash_end = index + 20;
        let hash = hex::encode(&content[index..hash_end]);
        index = hash_end;

        println!("{} {} {}", mode, hash, name);
    }
}