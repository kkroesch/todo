use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl std::str::FromStr for Priority {
    type Err = String;

    fn from_str(input: &str) -> Result<Priority, Self::Err> {
        match input.to_lowercase().as_str() {
            "low" => Ok(Priority::Low),
            "medium" => Ok(Priority::Medium),
            "high" => Ok(Priority::High),
            _ => Err(format!("Invalid priority: {}", input)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub due_date: Option<String>,
    pub finished: bool,
    pub priority: Priority,
    pub tags: Vec<String>,
    pub repeats: Option<String>,
}
