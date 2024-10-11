use crate::db::Database;
use clap::Args;

#[derive(Args)]
#[command(alias = "rm", about = "Remove task.")]
pub struct DeleteArgs {
    #[arg(index = 1, help = "Task number.")]
    pub id: String,
}

impl DeleteArgs {
    pub fn execute(&self) -> Result<String, Box<dyn std::error::Error>> {
        let db = Database::new()?;
        let key = format!("todo:{}", self.id);
        db.delete(&key)?;
        Ok("Removed".to_string())
    }
}
