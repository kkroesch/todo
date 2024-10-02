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
use crate::model::Todo;

#[derive(Args)]
#[command(about = "List unfinished task items.")]
pub struct ListArgs {
    #[arg(short = 'a', long = "all", help = "List finished tasks, too.")]
    pub all: bool,
}

impl ListArgs {
    pub fn execute(&self) -> Result<String, Box<dyn std::error::Error>> {
        let db_path = ".storage";
        let db = sled::open(db_path)?;

        let mut result = String::from("-- TODAY ------\n");
        for row in db.scan_prefix("todo:") {
            let (_, value) = row?;
            let todo: Todo = serde_json::from_slice(&value)?;
            result += &format!("[ ] {}\n", todo.title);
        }
        Ok(result)
    }
}
