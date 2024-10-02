use crate::db::list;
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
        let result = list("todo:")?;
        Ok(result)
    }
}
