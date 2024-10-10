use crate::db::Database;
use clap::Args;
use std::fmt;

#[derive(Debug)]
pub struct EmptyResultError;
impl fmt::Display for EmptyResultError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Empty result.")
    }
}
impl std::error::Error for EmptyResultError {}

#[derive(Args)]
#[command(alias = "ed", about = "Mark task as finished.")]
pub struct EditArgs {
    #[arg(index = 1, help = "Task number.")]
    pub id: String,
}

impl EditArgs {
    pub fn execute(&self) -> Result<String, Box<dyn std::error::Error>> {
        let db = Database::new(".storage")?;

        let start_key = format!("todo:0:0:{}", self.id);
        let end_key = format!("todo:0:2:{}", self.id);
        let range = start_key.as_bytes()..end_key.as_bytes();

        let result = db.range(range).unwrap();
        if result.len() == 0 {
            return Err(Box::new(EmptyResultError));
        }
        println!("{:#?}", result[0]);
        Ok("Edit".to_string())
    }
}
