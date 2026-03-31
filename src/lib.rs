pub mod models;
use std::{error::Error, fs, path::PathBuf};

use crate::models::{Cli, Commands, TodoItem, TodoList};

pub fn run(cli: Cli) -> Result<String, Box<dyn Error>> {
    match cli.command {
        Commands::Add { title } => {
            let mut list = load_list(&cli.store)?;
            list.items.push(TodoItem {
                title: title.clone(),
                done: false,
            });
            save_list(&cli.store, &list)?;
            Ok(format!("Added: {title}\n"))
        }
        Commands::List => {
            let list = load_list(&cli.store)?;
            if list.items.is_empty() {
                Ok("No items\n".to_string())
            } else {
                let mut out = String::new();
                for items in list.items {
                    out.push_str(&format!("- {}\n", items.title));
                }
                Ok(out)
            }
        },
        Commands::Done { id } => Ok(format!("{id}")),
    }
}

fn load_list(path: &PathBuf) -> Result<TodoList, Box<dyn std::error::Error>> {
    match fs::read_to_string(path) {
        Ok(contents) => Ok(serde_json::from_str(&contents)?),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(TodoList::default()),
        Err(e) => Err(e.into()),
    }
}

fn save_list(path: &PathBuf, list: &TodoList) -> Result<(), Box<dyn Error>> {
    let json = serde_json::to_string_pretty(list)?;
    fs::write(path, json)?;
    Ok(())
}

#[cfg(test)]
mod json_tests {
    use crate::models::{TodoItem, TodoList};

    #[test]
    fn todo_list_roundtrip_through_json() {
        let mut list = TodoList::default();
        list.items.push(TodoItem {
            title: "learn rust".to_string(),
            done: false,
        });

        let json = serde_json::to_string(&list).unwrap();
        let decoded: TodoList = serde_json::from_str(&json).unwrap();

        assert_eq!(decoded.items.len(), 1);
        assert_eq!(decoded.items[0].title, "learn rust");
    }
}
