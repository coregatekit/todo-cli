use clap::Parser;
use todo_cli::{Cli, Commands};

#[test]
fn parse_clean_command() {
    let cli = Cli::parse_from(["todo", "clean"]);
    match cli.command {
        Commands::Clean => {}
        _ => panic!("Expected Clean command"),
    }
}
