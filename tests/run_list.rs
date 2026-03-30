use clap::Parser;

#[test]
fn run_list_empty_returns_not_items() {
    let cli = todo_cli::Cli::parse_from(["todo", "list"]);
    let out = todo_cli::run(cli);
    assert_eq!(out, "No items\n");
}