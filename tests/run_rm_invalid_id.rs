use std::fs;

use clap::Parser;
use todo_cli::{Cli, TodoItem, TodoList};

#[test]
fn run_rm_invalid_id_returns_error() {
    let dir = tempfile::tempdir().unwrap();
    let store = dir.path().join("test.json");

    let list = TodoList {
      items: vec![
        TodoItem {
            title: "learn rust".to_string(),
            done: false,
        }
      ]
    };
    fs::write(&store, serde_json::to_string_pretty(&list).unwrap()).unwrap();

    let cli = Cli::parse_from(["todo", "--store", store.to_str().unwrap(), "rm", "2"]);
    let err = todo_cli::run(cli).unwrap_err();

    assert!(err.to_string().contains("Invalid item ID: 2"));
}