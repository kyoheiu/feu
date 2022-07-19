use super::errors::*;
use miniserde::{json, Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub paths: Vec<String>,
}

pub fn read_config() -> Config {
    if let Ok(config) = std::fs::read_to_string(config_path()) {
        if let Ok(deserialized) = json::from_str(&config) {
            deserialized
        } else {
            Config {
                paths: vec![String::from("/usr/bin")],
            }
        }
    } else {
        panic!("Cannot read the config file.");
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
    for path in config.paths {
        if let Ok(path) = PathBuf::from(path).canonicalize() {
            path_vec.push(path);
        }
    }
    path_vec
}

pub fn generate_bin_vec(path_vec: Vec<PathBuf>) -> Result<Vec<String>, FeuError> {
    let mut bin_vec = vec![];
    for path in path_vec {
        for bin in std::fs::read_dir(&path)? {
            let bin = bin?;
            if let Ok(name) = bin.file_name().into_string() {
                bin_vec.push(name);
            }
        }
    }
    Ok(bin_vec)
}
