use crate::db::{delete, insert};
use clap::Args;

#[derive(Args)]
#[command(alias = "fin", about = "Mark task as finished.")]
pub struct FinishArgs {
    #[arg(index = 1, help = "Task number.")]
    pub id: String,
}

impl FinishArgs {
    pub fn execute(&self) -> Result<String, Box<dyn std::error::Error>> {
        let key = format!("todo:0:0:{}", self.id);

        let mut old_task = delete(&key)?;
        old_task.finished = true;
        insert(old_task, true, 0)?;
        Ok("Updated".to_string())
    }
}
