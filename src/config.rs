use super::errors::*;
use miniserde::{json, Deserialize, Serialize};
use std::{collections::BTreeSet, path::PathBuf};

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

fn read_path() -> Result<BTreeSet<PathBuf>, FeuError> {
    let mut set = BTreeSet::new();
    let paths = std::env::var("PATH")?;
    for p in paths.split(':') {
        set.insert(p.to_string());
    }
    if let Ok(config) = std::fs::read_to_string(config_path()) {
        let deserialized: Config = json::from_str(&config)?;
        for p in deserialized.paths {
            set.insert(p);
        }
    }
    let set = set
        .iter()
        .filter_map(|x| PathBuf::from(x).canonicalize().ok())
        .collect();
    Ok(set)
}

pub fn generate_bin_set() -> Result<BTreeSet<String>, FeuError> {
    let mut bin_set = BTreeSet::new();
    for path in read_path()? {
        for bin in std::fs::read_dir(&path)? {
            let bin = bin?;
            if let Ok(name) = bin.file_name().into_string() {
                bin_set.insert(name);
            }
        }
    }
    Ok(bin_set)
}
