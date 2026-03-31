use clap::Parser;
use todo_cli::models::Cli;


fn main() {
    let cli = Cli::parse();
    match todo_cli::run(cli) {
        Ok(out)=>print!("{}", out),
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    }
}
