use std::error::Error;

use crate::{
    cli::{Cli, Commands},
    domain::TodoItem,
    store::{load_list, save_list},
};

pub fn run(cli: Cli) -> Result<String, Box<dyn Error>> {
    match cli.command {
        Commands::Add { title } => {
            let mut list = load_list(&cli.store)?;
            list.items.push(TodoItem {
                title: title.clone(),
                done: false,
            });
            save_list(&cli.store, &list)?;
            Ok(format!("Added: {title}\n"))
        }
        Commands::List => {
            let list = load_list(&cli.store)?;
            if list.items.is_empty() {
                Ok("No items\n".to_string())
            } else {
                let mut out = String::new();
                for (i, item) in list.items.iter().enumerate() {
                    let status = if item.done { "[x]" } else { "[ ]" };
                    out.push_str(&format!("{}. {} {}\n", i + 1, status, item.title));
                }
                Ok(out)
            }
        }
        Commands::Done { id } => {
            let mut list = load_list(&cli.store)?;

            if id == 0 || id > list.items.len() {
                return Err(format!("Invalid item ID: {id}").into());
            }

            let idx = id - 1;
            if list.items[idx].done {
                return Err(format!("Already done: {id}").into());
            }

            list.items[idx].done = true;
            save_list(&cli.store, &list)?;
            Ok(format!("Done: {id}\n"))
        }
        Commands::Rm { id } => {
            let mut list = load_list(&cli.store)?;

            if id == 0 || id > list.items.len() {
                return Err(format!("Invalid item ID: {id}").into());
            }

            let idx = id - 1;
            list.items.remove(idx);

            save_list(&cli.store, &list)?;
            Ok(format!("Removed: {id}\n"))
        },
        Commands::Clean => {
            let mut list = load_list(&cli.store)?;
            if list.items.is_empty() {
                return Ok("No items to clear\n".to_string());
            }
            let count = list.items.len();
            list.items.clear();
            save_list(&cli.store, &list)?;
            Ok(format!("Cleared {count} items\n"))
        },
    }
}
