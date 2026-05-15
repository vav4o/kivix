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

pub fn run(diff_size_p: Option<u64>, max_size: Option<u64>, max_size_p: Option<u64>) {
    println!("Setting diff config values.");

    let mut config = Config::load_config();
    config.diff_size_threshold_percentage = diff_size_p.unwrap_or(config.diff_size_threshold_percentage) as u64;
    config.max_accumulated_diff_size = max_size.unwrap_or(config.max_accumulated_diff_size) as u64;
    config.max_accumulated_diff_size_percentage = max_size_p.unwrap_or(config.max_accumulated_diff_size_percentage) as u64;
    config.save_config();

    println!("Diff config set successfully.");
}