
use crate::model::{Todo, Priority};

const CHECKMARK: &str = "\u{2713}";
const LOW_PRIO: &str = "\u{25cb}";
const MEDIUM_PRIO: &str = "\u{2710}";
const HIGH_PRIO: &str = "\u{2757}";

pub fn format(todo: Todo) -> String {
    return format!("[ ] {} ({})\n", todo.title, todo.id.chars().take(4).collect::<String>());
}