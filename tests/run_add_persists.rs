use std::fs;

use clap::Parser;
use todo_cli::Cli;

#[test]
fn run_add_persists_item_to_json_file() {
    let dir = tempfile::tempdir().unwrap();
    let store = dir.path().join("todo.json");

    let cli = Cli::parse_from([
      "todo",
      "--store",
      store.to_str().unwrap(),
      "add",
      "learn rust",
    ]);

    let out = todo_cli::run(cli).unwrap();
    assert_eq!(out, "Added: learn rust\n");

    let contents = fs::read_to_string(&store).unwrap();
    let decoded: todo_cli::models::TodoList = serde_json::from_str(&contents).unwrap();
    assert_eq!(decoded.items.len(), 1);
    assert_eq!(decoded.items[0].title, "learn rust");
}