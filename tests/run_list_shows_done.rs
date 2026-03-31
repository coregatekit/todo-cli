use std::fs;

use clap::Parser;
use todo_cli::models::{Cli, TodoItem, TodoList};

#[test]
fn list_shows_ids_and_done_status() {
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

    let cli = Cli::parse_from(["todo", "--store", store.to_str().unwrap(), "list"]);
    let out = todo_cli::run(cli).unwrap();

    assert_eq!(out, "1. [ ] learn rust\n2. [x] write tests\n");
}