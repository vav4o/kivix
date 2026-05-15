use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use serde::{Deserialize, Serialize};
use crate::tools::config_tools::LoadConfig;
use crate::commands::inner_diff;
use crate::commands::hash_object;
use crate::tools::hash_object as hash_object_tool;
use crate::tools::decoding::decode_content;
use crate::tools::read_file;
use std::io::Write;

#[derive(Serialize, Deserialize)]
struct DiffConfig {
    diff_size_threshold_percentage: u64,
    max_accumulated_diff_size: u64,
    max_accumulated_diff_size_percentage: u64,
}

impl LoadConfig for DiffConfig {}

//This function is way too big, however breaking it down would require decoding 
//the hashed file again which is inefficient
pub fn run(accumulated_diff_size: u64, file_path: &str, parent_hash: &str) -> (String, String) {
    
    let hashed_file_path= format!(".kiv/objects/{}/{}", &parent_hash[..2], &parent_hash[2..]);

    let hashed_file_data = std::fs::read(
        &hashed_file_path)
        .expect("Failed to read object file");
    
    let mut decoder = ZlibDecoder::new(&hashed_file_data[..]);
    let mut decompressed_data = Vec::new();
    std::io::copy(&mut decoder, &mut decompressed_data).unwrap();
    
    // Skip the object header, needs different size calculation as it is not encoded
    //(header format: "<type> <size>\0")
    let null_pos = decompressed_data.iter().position(|&b| b == 0x00)
        .expect("Invalid object format: missing null terminator");
    let hash_object_header_size = u64::try_from(null_pos + 1)
        .expect("Object header is too large to fit in u64");
    
    // Read the file from disk (as bytes, then decode with encoding detection)
    let file_data = std::fs::read(&file_path)
        .expect("Failed to read file");
    let mut file_encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    let file_encoded_payload = format!("blob {}\0", file_data.len())
        .into_bytes()
        .into_iter()
        .chain(file_data.iter().copied())
        .collect::<Vec<u8>>();
    file_encoder
        .write_all(&file_encoded_payload)
        .expect("Failed to compress file payload");
    let file_encoded = file_encoder.finish().expect("Failed to finish file compression");
    let file_size = u64::try_from(file_encoded.len())
        .expect("File size is too large to fit in u64");
    let file_content = decode_content(file_data);
    
    // Compute and return the diff
    let rendered = inner_diff::compute_diff(
        read_file::read_hashed_file(parent_hash), 
        file_content);
    
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&rendered).expect("Failed to compress rendered diff");
    let compressed_rendered = encoder.finish().expect("Failed to finish compression");
    let compressed_rendered_size = u64::try_from(compressed_rendered.len())
        .expect("Compressed rendered diff is too large to fit in u64");

    let total_size = hash_object_header_size + compressed_rendered_size;
    
    let config = DiffConfig::load_config();

    //println!("Diff:\n{}", String::from_utf8_lossy(&rendered));
    
    if total_size > file_size * config.diff_size_threshold_percentage / 100||
    total_size > config.max_accumulated_diff_size ||
    total_size + accumulated_diff_size > file_size * config.max_accumulated_diff_size_percentage / 100{
        println!("Output: {}, {}, file hash", total_size, file_size * config.diff_size_threshold_percentage / 100);
        return ("0".to_string(), hash_object::hash_file(&file_path, true));
    }
    
    let hash = hash_object_tool::hash_diff(rendered.clone(), parent_hash, true);
    println!("Output: {}, {}, diffff", total_size + accumulated_diff_size, file_size * config.diff_size_threshold_percentage / 100);
    return ((total_size + accumulated_diff_size).to_string(), hash);

    
//     println!("Total size (header + compressed diff): {} bytes", hash_object_header_size + compressed_rendered_size);
//     println!("Total accumulated diff size: {} bytes", accumulated_diff_size + hash_object_header_size + compressed_rendered_size);
//     println!("File size: {} bytes", file_size / 100 * config.diff_size_threshold_percentage);

}