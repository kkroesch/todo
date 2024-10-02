use crate::db::{insert, list};
use clap::Args;

#[derive(Args)]
#[command(alias = "ed", about = "Mark task as finished.")]
pub struct EditArgs {
    #[arg(index = 1, help = "Task number.")]
    pub id: String,
}

impl EditArgs {
    pub fn execute(&self) -> Result<String, Box<dyn std::error::Error>> {
        let key = format!("todo:{}", self.id);
        let result = list(&key, true).unwrap();
        println!("{:#?}", result[0]);
        Ok("Edit".to_string())
    }
}
