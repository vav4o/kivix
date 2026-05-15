use serde::{Deserialize, Serialize};
use std::fs;

pub trait LoadConfig: Sized + Serialize + for<'de> Deserialize<'de> {
    fn load_config() -> Self {
        let text = fs::read_to_string(".kiv/config.toml")
            .expect("Failed to read config file");

        toml::from_str(&text)
            .expect("Failed to parse config")
    }

     fn save_config(&self) {
        let toml_text = toml::to_string_pretty(self)
            .expect("Failed to serialize config");

        fs::write(".kiv/config.toml", toml_text)
            .expect("Failed to write config");
    }
}