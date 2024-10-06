use crate::db::{delete, insert, list};
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
        let result = list(&key).unwrap();
        for mut todo in result {
            todo.finished = true;
            // FIXME: DB layer should know about index keys, not commands.
            let full_key = format!("todo:0:0:{}", todo.id);
            insert(todo, true, 0)?;
            delete(&full_key)?;
        }
        Ok("Updated".to_string())
    }
}
