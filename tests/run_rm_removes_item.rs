use std::fs;

use clap::Parser;
use todo_cli::{Cli, TodoList};

#[test]
fn run_rm_removes_item_by_id() {
    let dir = tempfile::tempdir().unwrap();
    let store = dir.path().join("test.json");

    let list = TodoList {
      items: vec![
          todo_cli::domain::TodoItem {
              title: "learn rust".to_string(),
              done: false,
          },
          todo_cli::domain::TodoItem {
              title: "write tests".to_string(),
              done: false,
          },
      ],
    };
    fs::write(&store, serde_json::to_string_pretty(&list).unwrap()).unwrap();

    let cli = Cli::parse_from(["todo", "--store", store.to_str().unwrap(), "rm", "1"]);
    let out = todo_cli::run(cli).unwrap();
    assert_eq!(out, "Removed: 1\n");

    let contents = fs::read_to_string(&store).unwrap();
    let decoded: TodoList = serde_json::from_str(&contents).unwrap();

    assert_eq!(decoded.items[0].title, "write tests");
    assert_eq!(decoded.items[0].done, false);
    assert_eq!(decoded.items.len(), 1);
}