use hex;
use sha1::{Digest, Sha1};
use flate2::{write::ZlibEncoder, Compression};

use std::io::Write;

pub fn hash_object(data: Vec<u8>, object_type: &str, write: bool) -> String {
    let data: Vec<u8> = format!("{} {}\0", object_type, data.len())
        .into_bytes()
        .into_iter()
        .chain(data.into_iter())
        .collect::<Vec<u8>>();

    let mut hasher = Sha1::new();
    hasher.update(&data);
    let hash = hasher.finalize();
    let hexadecimal_hash = hex::encode(hash);

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&data).unwrap();
    let compressed_data = encoder.finish().unwrap();

    if write {
        let file_path: String = format!(".kiv/objects/{}/{}", &hexadecimal_hash[..2], &hexadecimal_hash[2..]);    

        std::fs::create_dir_all(std::path::Path::new(&file_path).parent().unwrap()).unwrap();
        std::fs::write(file_path, &compressed_data).unwrap();
    }

    hexadecimal_hash
}