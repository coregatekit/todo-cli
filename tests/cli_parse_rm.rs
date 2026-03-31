use clap::Parser;
use todo_cli::{Cli, Commands};

#[test]
fn parse_rm_command() {
    let cli = Cli::parse_from(["todo", "--store", "test.json", "rm", "1"]);
    match cli.command {
        Commands::Rm { id } => assert_eq!(id, 1),
        _ => panic!("Expected Rm command"),
    }
}
