use clap::Parser;
use todo_cli::{Cli, Commands};

#[test]
fn parse_add_command() {
    let cli = todo_cli::Cli::parse_from(["todo", "--store", "test.json", "add", "learn-rust"]);
    assert_eq!(cli.store.to_string_lossy(), "test.json");

    match cli.command {
        Commands::Add { title } => assert_eq!(title, "learn-rust"),
        _ => panic!("Expected Add command"),
    }
}

#[test]
fn parse_store_option() {
    let cli = Cli::parse_from(["todo", "--store", "my.json", "list"]);
    assert_eq!(cli.store.to_string_lossy(), "my.json");

    match cli.command {
        Commands::List => {},
        _ => panic!("Expected List command"),
    }
}
