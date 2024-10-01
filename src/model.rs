use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
enum Priority {
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
struct Todo {
    id: String,
    title: String,
    due_date: Option<String>,
    finished: bool,
    priority: Priority,
    tags: Vec<String>,
}
