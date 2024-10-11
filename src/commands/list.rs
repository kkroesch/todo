use crate::db::Database;
use crate::model::Todo;
use clap::Args;

#[derive(Args)]
#[command(alias = "ls", about = "List unfinished task items.")]
pub struct ListArgs {
    #[arg(short = 'a', long, help = "Also list finished tasks.")]
    pub all: bool,
}

impl ListArgs {
    pub fn execute(&self) -> Result<Vec<Todo>, Box<dyn std::error::Error>> {
        let db = Database::new()?;

        let from_prefix = "todo:0:0:";
        let end_prefix = "todo:9:1:";
        let prefix;
        if self.all {
            prefix = from_prefix.as_bytes()..end_prefix.as_bytes();
        } else {
            prefix = from_prefix.as_bytes().."todo:0:1:".as_bytes();
        }

        let result = db.range(prefix)?;
        Ok(result)
    }
}
