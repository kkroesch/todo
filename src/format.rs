use crate::model::{Priority, Todo};
use crossterm::style::Stylize;

/*
const CHECKMARK: &str = "\u{2713}";
const LOW_PRIO: &str = "\u{25cb}";
const MEDIUM_PRIO: &str = "\u{2710}";
const HIGH_PRIO: &str = "\u{2757}";
*/

pub fn format(todo: Todo) -> String {
    // Formatierung der Statusanzeige [ ] oder [✔]
    let status = if todo.finished {
        "[✔]".green()
    } else {
        "[ ]".white()
    };

    // Exclamation mark für hohe Priorität
    let priority_marker = match todo.priority {
        Priority::High => "!".red().bold(),
        _ => "".reset(),
    };

    // Erste 4 Zeichen der ID in Klammern, dunkler Schrift
    let id_display = format!("({})", &todo.id[..4]).dark_grey();

    // Titel in heller Schrift (Bright White)
    let title_display = todo.title.white();

    // Zusammensetzen der formatierten Zeile
    let line = format!(
        "{} {} {} {}\n",
        status, priority_marker, id_display, title_display
    );

    line
}
