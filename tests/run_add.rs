use clap::Parser;
use todo_cli::models::Cli;

#[test]
fn run_add_returns_added_line() {
    std::fs::remove_file("test.json").ok();
    let cli = Cli::parse_from(["todo", "--store", "test.json", "add", "learn-rust"]);
    let out = todo_cli::run(cli).unwrap();
    assert_eq!(out, "Added: learn-rust\n");
    std::fs::remove_file("test.json").ok();
}
