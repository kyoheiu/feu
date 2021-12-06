use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct History {
    pub history_map: HashMap<String, usize>,
}

pub fn read_history() -> Option<History> {
    let config = history_path();
    if let Ok(history) = std::fs::read_to_string(config) {
        let deserialized: History = toml::from_str(&history).unwrap();
        Some(deserialized)
    } else {
        None
    }
}

pub fn update_history(map: HashMap<String, usize>) -> std::io::Result<()> {
    let new_history = History { history_map: map };
    let toml = toml::to_string(&new_history).unwrap();
    std::fs::write(history_path(), toml)?;
    Ok(())
}

pub fn history_path() -> std::path::PathBuf {
    let mut history_path = dirs::home_dir().unwrap();
    history_path.push(".config");
    history_path.push("launcher");
    if !history_path.exists() {
        std::fs::create_dir_all(&history_path).unwrap();
    }
    history_path.push("history.toml");
    history_path
}
