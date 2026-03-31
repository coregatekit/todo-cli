use clap::Parser;
use todo_cli::Cli;

#[test]
fn run_list_empty_returns_not_items() {
    std::fs::remove_file("test.json").ok();
    let cli = Cli::parse_from(["todo", "--store", "test.json", "list"]);
    let out = todo_cli::run(cli);
    assert_eq!(out.unwrap(), "No items\n");
    std::fs::remove_file("test.json").ok();
}
