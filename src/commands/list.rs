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
use crate::db::list;

#[derive(Args)]
#[command(about = "List unfinished task items.")]
pub struct ListArgs {
    #[arg(short = 'a', long = "all", help = "List finished tasks, too.")]
    pub all: bool,
}

impl ListArgs {
    pub fn execute(&self) -> Result<String, Box<dyn std::error::Error>> {
        let result = list()?;
        Ok(result)
    }
}
