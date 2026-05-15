use serde::{Deserialize, Serialize};
use crate::tools::config_tools::LoadConfig;


#[derive(Serialize, Deserialize)]
struct Config {
    name: String,
    diff_size_threshold_percentage: u64,
    max_accumulated_diff_size: u64,
    max_accumulated_diff_size_percentage: u64,
}

impl LoadConfig for Config {}

pub fn run(name: String) {
    println!("Setting name to '{}'", name);

    let mut config = Config::load_config();
    config.name = name;
    config.save_config();

    println!("Name set successfully.");
}