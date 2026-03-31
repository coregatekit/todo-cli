use clap::Parser;
use todo_cli::models::{Cli, Commands};

#[test]
fn parse_done_command() {
    let cli = Cli::parse_from(["todo", "done", "2"]);
    match cli.command {
        Commands::Done { id } => assert_eq!(id, 2),
        _ => panic!("Expected Done command"),
    }
}