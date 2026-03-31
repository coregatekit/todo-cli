use std::fs;

use clap::Parser;
use todo_cli::models::{Cli, TodoItem, TodoList};

#[test]
fn run_list_prints_items_from_json_file() {
    let dir = tempfile::tempdir().unwrap();
    let store = dir.path().join("test.json");

    let list = TodoList {
        items: vec![
            TodoItem {
                title: "learn rust".to_string(),
            },
            TodoItem {
                title: "write tests".to_string(),
            },
        ],
    };

    fs::write(&store, serde_json::to_string_pretty(&list).unwrap()).unwrap();

    let cli = Cli::parse_from(["todo", "--store", store.to_str().unwrap(), "list"]);
    let out = todo_cli::run(cli).unwrap();

    assert_eq!(out, "- learn rust\n- write tests\n");
}
