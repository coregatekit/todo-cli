use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "todo")]
#[command(about = "A tiny todo CLI (TDD learning project)", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Add { title: String },
    List,
}

pub fn run(cli: Cli) -> String {
    match cli.command {
        Commands::Add { title } => format!("Added: {title}\n"),
        Commands::List => "No items\n".to_string(),
    }
}