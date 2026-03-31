use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct TodoList {
    pub items: Vec<TodoItem>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TodoItem {
    pub title: String,
    pub done: bool,
}

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
    Add {
        // Add a new todo item with the given title.
        title: String
    },
    List,
}
