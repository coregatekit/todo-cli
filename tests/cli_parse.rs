use clap::Parser;

#[test]
fn parse_add_command() {
    let _ = todo_cli::Cli::parse_from(["todo", "add", "Buy milk"]);
}