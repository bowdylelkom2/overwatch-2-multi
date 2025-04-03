use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Config {
    aimbot_sensitivity: f32,
    esp_enabled: bool,
    speed_multiplier: f32,
}

impl Config {
    pub fn new() -> Self {
        Config {
            aimbot_sensitivity: 1.0,
            esp_enabled: false,
            speed_multiplier: 1.0,
        }
    }

    pub fn load(file_path: &str) -> Self {
        let data = fs::read_to_string(file_path).unwrap_or_else(|_| String::new());
        serde_json::from_str(&data).unwrap_or_else(|_| Config::new())
    }

    pub fn save(&self, file_path: &str) {
        let data = serde_json::to_string(self).unwrap();
        fs::write(file_path, data).expect("Unable to write file");
    }
}