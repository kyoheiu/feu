use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub paths: Vec<String>,
}

pub fn read_config() -> Option<Config> {
    if let Ok(config) = std::fs::read_to_string(config_path()) {
        let deserialized: Config = ron::from_str(&config).unwrap();
        Some(deserialized)
    } else {
        None
    }
}

pub fn config_path() -> std::path::PathBuf {
    let mut config_path = dirs::config_dir().unwrap();
    config_path.push("feu");
    config_path.push("config");
    config_path
}
