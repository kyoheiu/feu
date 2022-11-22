mod application;
mod config;
mod errors;
mod history;
mod style;

use iced::window::Position;
use iced::Application;

const SIZE: (u32, u32) = (400, 224);

fn main() -> iced::Result {
    let window_setting = iced::window::Settings {
        size: SIZE,
        position: Position::Default,
        min_size: None,
        max_size: None,
        resizable: false,
        decorations: false,
        transparent: false,
        always_on_top: true,
        icon: None,
    };
    let setting = iced::Settings {
        window: window_setting,
        ..Default::default()
    };
    application::State::run(setting)
}

#[cfg(test)]
mod tests {
    use super::config::generate_bin_set;
    use super::history::*;
    use crate::config::Config;
    use std::collections::{BTreeMap, HashSet};
    use std::path::PathBuf;
    use std::time::Instant;

    #[test]
    fn perf_vec() {
        let instant = Instant::now();

        let history_path = history_path().unwrap_or_default();
        let history_map = if history_path.exists() {
            read_history(&history_path)
                .unwrap_or(History {
                    history_map: BTreeMap::new(),
                })
                .history_map
        } else {
            BTreeMap::new()
        };

        for _i in 0..1000 {
            let bin_source = generate_bin_set();
            if bin_source.is_err() {
                eprintln!("{:?}", bin_source.as_ref().unwrap_err());
            }
            let bin_source = bin_source.unwrap();

            let mut used_bins = vec![];
            let mut unused_bins: Vec<(String, usize)> = vec![];

            // clone() to align conditions with perf_set
            if !history_map.clone().is_empty() {
                for b in bin_source {
                    match history_map.get(&b) {
                        Some(i) => {
                            used_bins.push((b, *i));
                        }
                        None => {
                            unused_bins.push((b, 0));
                        }
                    }
                }
                used_bins.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                used_bins.append(&mut unused_bins);
            } else {
                used_bins = bin_source.iter().map(|x| (x.to_string(), 0)).collect();
            }
        }

        let duration = instant.elapsed();
        println!("vec: {}s", duration.as_secs_f32());
    }

    #[test]
    fn perf_set() {
        let instant = Instant::now();

        let history_path = history_path().unwrap_or_default();
        let history_map = if history_path.exists() {
            read_history(&history_path)
                .unwrap_or(History {
                    history_map: BTreeMap::new(),
                })
                .history_map
        } else {
            BTreeMap::new()
        };

        for _i in 0..1000 {
            let mut bin_source = test_generate_bin_set();

            let mut used_bins = vec![];
            let mut unused_bins: Vec<(String, usize)> = vec![];

            for h in history_map.clone() {
                match bin_source.get(&h.0) {
                    Some(_) => {
                        bin_source.remove(&h.0);
                        used_bins.push(h);
                    }
                    None => {}
                }
            }
            for bin in bin_source {
                unused_bins.push((bin, 0));
            }

            used_bins.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            used_bins.append(&mut unused_bins);
        }

        let duration = instant.elapsed();
        println!("set: {}s", duration.as_secs_f32());
    }

    fn config_path() -> std::path::PathBuf {
        let mut config_path = dirs::config_dir().unwrap();
        config_path.push("feu");
        config_path.push("config.json");
        config_path
    }

    fn read_path_set() -> HashSet<String> {
        let paths = std::env::var("PATH").unwrap();
        let mut set = HashSet::new();
        for path in paths.split(':') {
            set.insert(path.to_string());
        }
        if let Ok(config) = std::fs::read_to_string(config_path()) {
            if let Ok::<Config, miniserde::Error>(deserialized) = miniserde::json::from_str(&config)
            {
                for p in deserialized.paths {
                    set.insert(p);
                }
            }
        }
        set
    }

    fn generate_path_set() -> Vec<PathBuf> {
        let config = read_path_set();
        let mut path_vec = vec![];
        for path in config {
            if let Ok(path) = PathBuf::from(path).canonicalize() {
                path_vec.push(path);
            }
        }
        path_vec
    }

    pub fn test_generate_bin_set() -> HashSet<String> {
        let mut bin_set = HashSet::new();
        for path in generate_path_set() {
            for bin in std::fs::read_dir(&path).unwrap() {
                let bin = bin.unwrap();
                if let Ok(name) = bin.file_name().into_string() {
                    bin_set.insert(name);
                }
            }
        }
        bin_set
    }
}
