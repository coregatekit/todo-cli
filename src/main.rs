use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "todo")]
#[command(about = "A simple CLI todo application", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add { title: String },
    List,    
}

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
