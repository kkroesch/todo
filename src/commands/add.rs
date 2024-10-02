use crate::model::{Priority, Todo};
use clap::Args;
use uuid::Uuid;

use crate::db::insert;

#[derive(Args)]
#[command(alias = "a", about = "Add a task item.")]
pub struct AddArgs {
    #[arg(index = 1, help = "Title for the task.")]
    pub title: String,
    #[arg(short = 'p', long, help = "Priority. One of 'low', 'medium', 'high'.")]
    pub priority: Option<Priority>,
}

impl AddArgs {
    pub fn execute(&self) -> Result<String, Box<dyn std::error::Error>> {
        let todo = Todo {
            id: Uuid::new_v4().to_string(),
            title: self.title.to_string(),
            due_date: Some("today".to_string()),
            finished: false,
            priority: Priority::Medium,
            tags: vec![],
        };

        insert(todo)?;

        Ok(format!("{}", "Added todo."))
    }
}
