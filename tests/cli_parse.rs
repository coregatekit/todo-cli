use clap::Parser;
use todo_cli::{Cli, Commands};

#[test]
fn parse_add_command() {
    let cli = Cli::parse_from(["todo", "add", "learn-rust"]);
    assert_eq!(cli.store, None);

    match cli.command {
        Commands::Add { title } => assert_eq!(title, "learn-rust"),
        _ => panic!("Expected Add command"),
    }
}

#[test]
fn parse_store_option() {
    let cli = Cli::parse_from(["todo", "list"]);
    assert_eq!(cli.store, None);

    match cli.command {
        Commands::List => {}
        _ => panic!("Expected List command"),
    }
}
