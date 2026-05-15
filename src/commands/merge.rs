use crate::tools::merging::three_way_merge;

pub fn run(base_file: String, my_file: String, their_file: String) {
    let merged_content = three_way_merge(base_file, my_file, their_file);

    std::fs::write("merged_result.txt", &merged_content).expect("Failed to write merged result to file");
    println!("Merged result written to merged_result.txt");

}