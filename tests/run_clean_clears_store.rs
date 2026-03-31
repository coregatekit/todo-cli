use std::fs;

use clap::Parser;
use todo_cli::{Cli, TodoItem, TodoList};

#[test]
fn run_clean_clears_all_items() {
    let dir = tempfile::tempdir().unwrap();
    let store = dir.path().join("test.json");

    let list = TodoList {
        items: vec![
            TodoItem {
                title: "learn rust".to_string(),
                done: false,
            },
            TodoItem {
                title: "write tests".to_string(),
                done: true,
            },
        ],
    };
    fs::write(&store, serde_json::to_string_pretty(&list).unwrap()).unwrap();

    let cli = Cli::parse_from(["todo", "--store", store.to_str().unwrap(), "clean"]);
    let out = todo_cli::run(cli).unwrap();
    assert_eq!(out, "Cleared 2 items\n");

    let contents = fs::read_to_string(&store).unwrap();
    let decoded: TodoList = serde_json::from_str(&contents).unwrap();
    assert!(decoded.items.is_empty());
}