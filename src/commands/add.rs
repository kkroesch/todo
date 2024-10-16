use std::str::FromStr;

use crate::model::{Priority, Todo};
use clap::Args;

use crate::db::Database;

#[derive(Args)]
#[command(alias = "a", about = "Add a task item.")]
pub struct AddArgs {
    #[arg(index = 1, help = "Title for the task.")]
    pub title: String,
    #[arg(short = 'p', long, help = "Priority. One of 'low', 'medium', 'high'.")]
    pub priority: Option<String>,
    #[arg(
        short = 'd',
        long,
        help = "Timespan to remember. For example: '2d' or 'next friday'."
    )]
    pub due_date: Option<String>,
    #[arg(
        short = 't',
        long, num_args = 1.., value_name = "TAG",
        value_delimiter = ',',
        help = "A tag for task. Can be applied more than once."
    )]
    pub tags: Vec<String>,
    #[arg(
        short = 'r',
        long,
        help = "Timespan to repeat. For example: '1w' or 'monday'."
    )]
    pub repeats: Option<String>,
}

impl AddArgs {
    pub fn execute(&self) -> Result<String, Box<dyn std::error::Error>> {
        let todo = Todo::new(
            self.title.to_string(),
            Priority::from_str(self.priority.as_deref().unwrap_or("medium")).unwrap(),
        );
        let db = Database::new()?;
        db.insert(todo, false, 0)?;

        Ok(format!("{}", "Added todo."))
    }
}
