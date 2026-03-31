use clap::Parser;
use todo_cli::{Cli, models::TodoList};

#[test]
fn add_accepts_multiword_title_without_quotes() {
    let dir = tempfile::tempdir().unwrap();
    let store = dir.path().join("test.json");

    let cli = Cli::parse_from([
      "todo",
      "--store",
      store.to_str().unwrap(),
      "add",
      "lear",
      "rust",
      "with",
      "tdd"
    ]);

    let out = todo_cli::run(cli).unwrap();
    assert_eq!(out, "Added: lear rust with tdd\n");

    let contents = std::fs::read_to_string(&store).unwrap();
    let decoded: TodoList = serde_json::from_str(&contents).unwrap();
    assert_eq!(decoded.items[0].title, "learn rust with tdd");
}