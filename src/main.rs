//! # Todo-App
//!
//! Manage to-do items on the command line. Supports various backends.
//!
//! Default backend is the [Sled Database](https://sled.rs).
//! The backend code demonstrates efficient handling of a key-value store.
//!
//! ## Available Commands
//!
//! - `add`: Adds a new task item.
//! - `list`: List all tasks.
//! - `finish`: Mark a task as finished.
//! - `edit`: Edit a tasks fields.
//! - `delete`: Remove a task.
//! - `postpone`: Move a tasks due date to the future.
//!
//!
//! ## License and Copyright
//!
//! This code is written by Karsten Kroesch <kkroesch@me.com>
//! and published under the terms of the MIT license.
//!

use clap::{Parser, Subcommand};
use crossterm::execute;
use crossterm::style::Print;
use std::io;

mod commands;
mod config;
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
    Postpone(commands::postpone::PostponeArgs),
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Add(args) => {
            let result = args.execute();
            format::ok(result.unwrap());
        }
        Commands::List(args) => {
            let result = args.execute();
            println!("== TODAY ================");
            for todo in result.unwrap() {
                let line = format::format(todo);
                execute!(io::stdout(), Print(line)).unwrap();
            }
        }
        Commands::Finish(args) => {
            let result = args.execute();
            format::ok(result.unwrap());
        }
        Commands::Edit(args) => match args.execute() {
            Ok(message) => format::ok(message),
            Err(err) => format::error(format!("{err}")),
        },
        Commands::Delete(args) => {
            let line = args.execute().unwrap();
            format::ok(line);
        }
        Commands::Postpone(args) => {
            let line = args.execute().unwrap();
            format::ok(line);
        }
    }
}
