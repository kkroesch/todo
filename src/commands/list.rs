use crate::db::list;
use crate::model::Todo;
/// List task items from database with various filter functions.
///
/// # Arguments
///
///
/// # Result
///
/// Formatted list with task items.
///
/// # Example
///
/// ```
/// let result = add(5, 10);
/// assert_eq!(result, 15);
/// ```
use clap::Args;

#[derive(Args)]
#[command(alias = "ls", about = "List unfinished task items.")]
pub struct ListArgs {
    #[arg(short = 'a', long = "all", help = "List finished tasks, too.")]
    pub all: bool,
}

impl ListArgs {
    pub fn execute(&self) -> Result<Vec<Todo>, Box<dyn std::error::Error>> {
        let result = list("todo:")?;
        Ok(result)
    }
}
