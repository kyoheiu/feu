use std::path::PathBuf;

use crate::errors::FeuError;

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
