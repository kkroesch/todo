use crate::db::Database;
use crate::model::Todo;
use clap::Args;

#[derive(Args)]
#[command(alias = "ls", about = "List unfinished task items.")]
pub struct ListArgs {
    #[arg(short = 'a', long = "all", help = "Also list finished tasks.")]
    pub all: bool,
}

impl ListArgs {
    pub fn execute(&self) -> Result<Vec<Todo>, Box<dyn std::error::Error>> {
        let db = Database::new(".storage")?;

        let done_prefix = if self.all { "" } else { "0" };
        let prefix = format!("todo:0:{}", done_prefix);
        let result = db.list(&prefix)?;
        Ok(result)
    }
}
