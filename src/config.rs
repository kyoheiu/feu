use super::errors::*;
use miniserde::{json, Deserialize, Serialize};
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

fn read_path() -> Result<Vec<PathBuf>, FeuError> {
    let mut v = Vec::new();
    let paths = std::env::var("PATH")?;
    for p in paths.split(':') {
        if !v.contains(&p.to_string()) {
            v.push(p.to_string());
        }
    }
    if let Ok(config) = std::fs::read_to_string(config_path()) {
        let deserialized: Config = json::from_str(&config)?;
        for p in deserialized.paths {
            if !v.contains(&p) {
                v.push(p);
            }
        }
    }
    let mut v: Vec<PathBuf> = v
        .iter()
        .filter_map(|x| PathBuf::from(x).canonicalize().ok())
        .collect();
    v.sort();
    v.dedup();
    Ok(v)
}

pub fn generate_bin_vec() -> Result<Vec<String>, FeuError> {
    let mut bin_v = Vec::new();
    for path in read_path()? {
        for bin in std::fs::read_dir(&path)? {
            let bin = bin?;
            if let Ok(name) = bin.file_name().into_string() {
                bin_v.push(name);
            }
        }
    }
    Ok(bin_v)
}
