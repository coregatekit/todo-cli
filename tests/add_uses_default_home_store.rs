use clap::Parser;
use todo_cli::{Cli, TodoList};

#[test]
fn add_writes_to_home_default_store_when_store_not_provide() {
    let home = tempfile::tempdir().unwrap();
    unsafe { std::env::set_var("HOME", home.path()) };

    let cli = Cli::parse_from(["todo", "add", "learn-rust"]);
    let out = todo_cli::run(cli).unwrap();
    assert_eq!(out, "Added: learn-rust\n");

    let store = home.path().join(".todo_cli").join("todo.json");
    assert!(store.exists());

    let contents = std::fs::read_to_string(store).unwrap();
    let decoded: TodoList = serde_json::from_str(&contents).unwrap();
    assert_eq!(decoded.items.len(), 1);
}