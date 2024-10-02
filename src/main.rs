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

mod commands;
mod model;
mod db;

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
            println!("{}", result.unwrap());
        }
        Commands::Finish(args) => {
            let result = args.execute();
            println!("{}", result.unwrap());
        }
    }
}
