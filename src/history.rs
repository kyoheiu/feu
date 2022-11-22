use super::errors::FeuError;
use miniserde::{json, Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize)]
pub struct History {
    pub history_map: BTreeMap<String, usize>,
}

pub fn read_history(path: &Path) -> Result<History, FeuError> {
    if let Ok(history) = std::fs::read_to_string(path) {
        let deserialized: History = json::from_str(&history)?;
        Ok(deserialized)
    } else {
        Ok(History {
            history_map: BTreeMap::new(),
        })
    }
}

pub fn update_history(map: &BTreeMap<String, usize>, path: &Path) -> Result<(), FeuError> {
    let new_history = History {
        history_map: map.clone(),
    };
    let new_history = json::to_string(&new_history);
    std::fs::write(path, new_history)?;
    Ok(())
}

pub fn history_path() -> Result<PathBuf, FeuError> {
    let mut history_path = dirs::config_dir().unwrap();
    history_path.push("feu");
    if !history_path.exists() {
        std::fs::create_dir_all(&history_path)?;
    }
    history_path.push(".history");
    Ok(history_path)
}
