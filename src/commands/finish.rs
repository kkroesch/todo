use crate::db::{insert, list};
use clap::Args;

#[derive(Args)]
#[command(alias = "fin", about = "Mark task as finished.")]
pub struct FinishArgs {
    #[arg(index = 1, help = "Task number.")]
    pub id: String,
}

impl FinishArgs {
    pub fn execute(&self) -> Result<String, Box<dyn std::error::Error>> {
        let key = format!("todo:{}", self.id);
        let result = list(&key, false).unwrap();
        for mut todo in result {
            todo.finished = true;
            insert(todo)?;
        }
        Ok("Updated".to_string())
    }
}
