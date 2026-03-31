use std::fs;

use clap::Parser;
use todo_cli::models::{Cli, TodoList};

#[test]
fn run_done_marks_item_as_done() {
    let dir = tempfile::tempdir().unwrap();
    let store = dir.path().join("test.json");

    let list = TodoList {
      items: vec![
        todo_cli::models::TodoItem {
          title: "learn rust".to_string(),
          done: false,
        },
        todo_cli::models::TodoItem {
          title: "write tests".to_string(),
          done: false,
        },
      ],
    };

    fs::write(&store, serde_json::to_string_pretty(&list).unwrap()).unwrap();

    let cli = Cli::parse_from(["todo", "--store", store.to_str().unwrap(), "done", "2"]);
    let out = todo_cli::run(cli).unwrap();
    assert_eq!(out, "Done: 2\n");

    let contents = fs::read_to_string(&store).unwrap();
    let decoded: TodoList = serde_json::from_str(&contents).unwrap();
    assert_eq!(decoded.items.len(), 2);
    assert_eq!(decoded.items[0].title, "learn rust");
    assert_eq!(decoded.items[0].done, false);
    assert_eq!(decoded.items[1].title, "write tests");
    assert_eq!(decoded.items[1].done, true);
}

#[test]
fn run_done_marks_item_as_done_but_invalid_id() {
    let dir = tempfile::tempdir().unwrap();
    let store = dir.path().join("test.json");

    let list = TodoList {
      items: vec![
        todo_cli::models::TodoItem {
          title: "learn rust".to_string(),
          done: false,
        },
        todo_cli::models::TodoItem {
          title: "write tests".to_string(),
          done: false,
        },
      ],
    };

    fs::write(&store, serde_json::to_string_pretty(&list).unwrap()).unwrap();

    let cli = Cli::parse_from(["todo", "--store", store.to_str().unwrap(), "done", "3"]);
    let out = todo_cli::run(cli);
    assert!(out.is_err());
    assert_eq!(out.err().unwrap().to_string(), "Invalid item ID: 3");

    let contents = fs::read_to_string(&store).unwrap();
    let decoded: TodoList = serde_json::from_str(&contents).unwrap();
    assert_eq!(decoded.items.len(), 2);
    assert_eq!(decoded.items[0].title, "learn rust");
    assert_eq!(decoded.items[0].done, false);
    assert_eq!(decoded.items[1].title, "write tests");
    assert_eq!(decoded.items[1].done, false);
}