pub fn run(_pretty_print: bool, object_hash: String) {
    let content = crate::tools::read_file::read_hashed_file(&object_hash);
    println!("{}", content);
}