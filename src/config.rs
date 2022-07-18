use miniserde::{json, Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub paths: Vec<String>,
}

pub fn read_config() -> Option<Config> {
    if let Ok(config) = std::fs::read_to_string(config_path()) {
        let deserialized: Config = json::from_str(&config).unwrap();
        Some(deserialized)
    } else {
        None
    }
}

pub fn config_path() -> std::path::PathBuf {
    let mut config_path = dirs::config_dir().unwrap();
    config_path.push("feu");
    config_path.push("config.json");
    config_path
}

pub fn generate_path_vec() -> Vec<PathBuf> {
    let config = read_config();
    let mut path_vec = vec![];
    match config {
        Some(config) => {
            for path in config.paths {
                if let Ok(path) = PathBuf::from(path).canonicalize() {
                    path_vec.push(path);
                }
            }
        }
        None => {
            path_vec.push(std::path::PathBuf::from("/usr/bin"));
        }
    }
    path_vec
}
