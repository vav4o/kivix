use flate2::read::ZlibDecoder;
use std::fs::File;
use std::io::Read;
use std::io::Result;

const TREE_BYTES: [u8; 5] = [b't', b'r', b'e', b'e', b' '];

pub fn run(object_hash: String) {
    let tree = get_tree(&object_hash);
    for entry in tree {
        println!("{} {} {}", entry.mode, hex::encode(entry.hash), entry.name);
            
        
        
    }
}

struct TreeEntry {
    name: String,
    hash: Vec<u8>,
    mode: u32,
}

fn get_tree(hash: &str) -> Vec<TreeEntry> {
    let file = File::open(format!(".kiv/objects/{}/{}", &hash[..2], &hash[2..])).unwrap();
    let result = decode(file).unwrap();
    extract_tree(result)
}

fn decode(file: File) -> Result<Vec<u8>> {
    let mut decoder = ZlibDecoder::new(file);
    let mut data = Vec::new();
    decoder.read_to_end(&mut data)?;
    Ok(data)
}

fn extract_tree(data: Vec<u8>) -> Vec<TreeEntry> {
    if !data.starts_with(&TREE_BYTES) {
        panic!("not a tree object");
    }
    
    let mut i = TREE_BYTES.len();
    let null_pos = data[i..].iter().position(|&x| x == 0).unwrap();
    let content_size = std::str::from_utf8(&data[i..i + null_pos])
    .unwrap()
    .parse::<usize>()
    .unwrap();
    i += null_pos + 1;

    let mut entries = Vec::new();
    let content_end = i + content_size;

    while i < content_end {
        // Parse mode
        let space_pos = data[i..content_end].iter().position(|&b| b == b' ').map(|p| p + i).expect("Invalid tree entry: missing space for mode");
        let mode = std::str::from_utf8(&data[i..space_pos])
            .unwrap()
            .parse()
            .unwrap();
        i = space_pos + 1;

        // Parse name
        let null_pos = data[i..content_end].iter().position(|&b| b == 0).map(|p| p + i).expect("Invalid tree entry: missing null for name");
        let name = std::str::from_utf8(&data[i..null_pos]).unwrap().to_string();
        i = null_pos + 1;

        // Parse hash
        let hash = data[i..i + 20].to_vec();
        i += 20;

        entries.push(TreeEntry { mode, name, hash });
    }

    entries
}