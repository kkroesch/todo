use crate::db::list;
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
        let key = format!("todo:0:0:{}", self.id);
        let result = list(&key).unwrap();
        if result.len() == 0 {
            return Err(Box::new(EmptyResultError));
        }
        println!("{:#?}", result[0]);
        Ok("Edit".to_string())
    }
}
