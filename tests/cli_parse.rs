use clap::Parser;
use todo_cli::Cli;

#[test]
fn parse_add_command() {
    let _ = todo_cli::Cli::parse_from(["todo", "add", "Buy milk"]);
}

#[test]
fn run_add_returns_added_line() {
    let cli = Cli::parse_from(["todo", "add", "learn-rust"]);
    let out = todo_cli::run(cli);    
    assert!(out.contains("Added: learn rust\n"));
}