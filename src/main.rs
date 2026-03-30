use clap::Parser;
use todo_cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { title } => {
            println!("Adding todo: {}", title);
            // Here you would add the logic to save the todo item
        }
        Commands::List => {
            println!("Listing all todos...");
            // Here you would add the logic to retrieve and display all todo items
        }
    }
}
