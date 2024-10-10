use crate::db::Database;
use clap::Args;

#[derive(Args)]
#[command(alias = "pp", about = "Move due of the task into the future.")]
pub struct PostponeArgs {
    #[arg(index = 1, help = "Task number.")]
    pub id: String,

    #[arg(index = 2, help = "Number of days.")]
    pub days: u8,
}

impl PostponeArgs {
    pub fn execute(&self) -> Result<String, Box<dyn std::error::Error>> {
        let db = Database::new(".storage")?;
        let key = format!("todo:0:0:{}", self.id);
        let result = db.list(&key).unwrap();
        for mut todo in result {
            todo.due_date = Some(format!("{}", self.days));
        }
        Ok("Updated".to_string())
    }
}
