use clap::Parser;
use todo_cli::Cli;

#[test]
fn run_add_returns_added_line() {
    let cli = Cli::parse_from(["todo", "add", "learn-rust"]);
    let out = todo_cli::run(cli);
    assert_eq!(out.unwrap(), "Added: learn-rust\n");
}
