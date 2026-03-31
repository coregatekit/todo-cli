use std::{error::Error, fs, path::PathBuf};

use crate::domain::TodoList;

pub fn load_list(path: &PathBuf) -> Result<TodoList, Box<dyn Error>> {
    match fs::read_to_string(path) {
        Ok(contents) => Ok(serde_json::from_str(&contents)?),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(TodoList::default()),
        Err(e) => Err(e.into()),
    }
}

pub fn save_list(path: &PathBuf, list: &TodoList) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(list)?;
    fs::write(path, json)?;
    Ok(())
}
