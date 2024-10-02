use crate::model::{Priority, Todo};
use crossterm::style::Stylize;

const LOW_PRIO: &str = "\u{25cb}";
const MEDIUM_PRIO: &str = "\u{2710}";
const HIGH_PRIO: &str = "!";

pub fn format(todo: Todo) -> String {
    // Formatierung der Statusanzeige [ ] oder [✔]
    let status = if todo.finished {
        "[✔]".green()
    } else {
        "[ ]".white()
    };

    // Exclamation mark für hohe Priorität
    let priority_marker = match todo.priority {
        Priority::High => HIGH_PRIO.red(),
        Priority::Medium => MEDIUM_PRIO.blue(),
        Priority::Low => LOW_PRIO.white(),
    };

    // Erste 4 Zeichen der ID in Klammern, dunkler Schrift
    let id_display = format!("({})", &todo.id[..4]).dark_grey();

    // Titel in heller Schrift (Bright White)
    let title_display = todo.title.white().bold();

    // Tags in hellblau mit Hash
    let tag_line = todo
        .tags
        .iter()
        .map(|tag| format!("#{}", tag))
        .collect::<Vec<String>>()
        .join(" ");

    // Zusammensetzen der formatierten Zeile
    let line = format!(
        "{} {} {} {} {}\n",
        status, id_display, priority_marker, title_display, tag_line
    );

    line
}
