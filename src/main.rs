//! # Todo-App
//!
//! Manage to-do items on the command line. Supports various backends.
//!
//! ## Available Commands
//!
//! - `add`: Adds a new task item.
//! - `list`: List all tasks.
//! - `finish`: Mark a task as finished.

use clap::{Parser, Subcommand};
use crossterm::execute;
use crossterm::style::Print;
use std::io;

mod commands;
mod db;
mod format;
mod model;

#[derive(Parser)]
#[command(
    name = "Todo App",
    version = "1.0",
    author = "Karsten Kroesch",
    about = "Manage to-do items on the command line."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add(commands::add::AddArgs),
    List(commands::list::ListArgs),
    Finish(commands::finish::FinishArgs),
    Edit(commands::edit::EditArgs),
    Delete(commands::delete::DeleteArgs),
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add(args) => {
            let result = args.execute();
            println!("{}", result.unwrap());
        }
        Commands::List(args) => {
            let result = args.execute();
            for todo in result.unwrap() {
                let line = format::format(todo);
                execute!(io::stdout(), Print(line)).unwrap();
            }
        }
        Commands::Finish(args) => {
            let result = args.execute();
            let line = result.unwrap();
            execute!(io::stdout(), Print(line)).unwrap();
        }
        Commands::Edit(args) => {
            args.execute().unwrap();
        }
        Commands::Delete(args) => {
            let line = args.execute().unwrap();
            execute!(io::stdout(), Print(line)).unwrap();
        }
    }
}
