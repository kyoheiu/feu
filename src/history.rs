use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct History {
    pub history_map: HashMap<String, usize>,
}

pub fn read_history(path: &std::path::Path) -> Option<History> {
    if let Ok(history) = std::fs::read_to_string(path) {
        let deserialized: History = ron::from_str(&history).unwrap();
        Some(deserialized)
    } else {
        None
    }
}

pub fn update_history(map: &HashMap<String, usize>, path: &std::path::Path) -> std::io::Result<()> {
    let new_history = History {
        history_map: map.clone(),
    };
    let toml = ron::to_string(&new_history).unwrap();
    std::fs::write(path, toml)?;
    Ok(())
}

pub fn history_path() -> std::path::PathBuf {
    let mut history_path = dirs::config_dir().unwrap();
    history_path.push("feu");
    if !history_path.exists() {
        std::fs::create_dir_all(&history_path).unwrap();
    }
    history_path.push(".history");
    history_path
}
