use super::errors::*;
use miniserde::{json, Deserialize, Serialize};
use std::collections::HashSet;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub paths: Vec<String>,
}

fn config_path() -> std::path::PathBuf {
    let mut config_path = dirs::config_dir().unwrap();
    config_path.push("feu");
    config_path.push("config.json");
    config_path
}

fn read_path() -> HashSet<String> {
    let paths = std::env::var("PATH").unwrap();
    let mut set = HashSet::new();
    for path in paths.split(':') {
        set.insert(path.to_string());
    }
    if let Ok(config) = std::fs::read_to_string(config_path()) {
        if let Ok::<Config, miniserde::Error>(deserialized) = json::from_str(&config) {
            for p in deserialized.paths {
                set.insert(p);
            }
        }
    }
    set
}

fn generate_path_vec() -> Vec<PathBuf> {
    let config = read_path();
    let mut path_vec = vec![];
    for path in config {
        if let Ok(path) = PathBuf::from(path).canonicalize() {
            path_vec.push(path);
        }
    }
    path_vec
}

pub fn generate_bin_vec() -> Result<Vec<String>, FeuError> {
    let mut bin_vec = vec![];
    for path in generate_path_vec() {
        for bin in std::fs::read_dir(&path)? {
            let bin = bin?;
            if let Ok(name) = bin.file_name().into_string() {
                bin_vec.push(name);
            }
        }
    }
    Ok(bin_vec)
}
