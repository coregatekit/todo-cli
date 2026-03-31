pub mod models;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "todo")]
#[command(about = "A tiny todo CLI (TDD learning project)", long_about = None)]
pub struct Cli {
    // Path to store json file
    #[arg(long, default_value = "todo.json")]
    pub store: std::path::PathBuf,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Add { title: String },
    List,
}

pub fn run(cli: Cli) -> String {
    match cli.command {
        Commands::Add { title } => format!("Added: {title}\n"),
        Commands::List => "No items\n".to_string(),
    }
}

#[cfg(test)]
mod json_tests {
    use crate::models::{TodoItem, TodoList};

    #[test]
    fn todo_list_roundtrip_through_json() {
        let mut list = TodoList::default();
        list.items.push(TodoItem {
            title: "learn rust".to_string(),
        });

        let json = serde_json::to_string(&list).unwrap();
        let decoded: TodoList = serde_json::from_str(&json).unwrap();

        assert_eq!(decoded.items.len(), 1);
        assert_eq!(decoded.items[0].title, "learn rust");
    }
}
