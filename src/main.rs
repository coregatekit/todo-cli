use clap::Parser;
use todo_cli::{Cli};

fn main() {
    let cli = Cli::parse();
    let out = todo_cli::run(cli);
    print!("{out}");
}
