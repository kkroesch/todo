/// Führt eine Berechnung durch und gibt das Ergebnis zurück.
///
/// # Argumente
///
/// * `x` - Ein 32-Bit Integer-Wert.
/// * `y` - Ein 32-Bit Integer-Wert.
///
/// # Rückgabewert
///
/// Gibt die Summe von `x` und `y` zurück.
///
/// # Beispiel
///
/// ```
/// let result = add(5, 10);
/// assert_eq!(result, 15);
/// ```
use clap::Args;
use crate::db::list;
use crate::format::format;
use crate::model::Todo;

#[derive(Args)]
#[command(about = "Mark task as finished.")]
pub struct FinishArgs {
    #[arg(short, long, help = "Task number.")]
    pub id: String,
}

impl FinishArgs {
    pub fn execute(&self) -> Result<Vec<Todo>, Box<dyn std::error::Error>> {
        let key = format!("todo:{}", self.id);
        let mut result= list("todo:").unwrap();
        for todo in result {
            todo.finished = true;
        }
        Ok(result)
    }
}
