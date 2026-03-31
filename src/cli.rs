use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "todo")]
#[command(about = "A tiny todo CLI (TDD learning project)", long_about = None)]
pub struct Cli {
    /// Path to store json file
    #[arg(long, value_name = "PATH")]
    pub store: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Add a new todo item with the given title. e.g. `todo add "learn rust"`
    Add {
        // Add a new todo item with the given title.
        title: String,
    },
    /// List all todo items, showing their ID, title, and completion status. e.g. `todo list`
    List,
    /// Mark a todo item as done by its ID. e.g. `todo done 1`
    Done {
        id: usize,
    },
    /// Remove a todo item by its ID. e.g. `todo rm 1`
    Rm {
        id: usize,
    },
    /// Clear all todo items. e.g. `todo clean`
    Clean,
}
