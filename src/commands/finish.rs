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

#[derive(Args)]
#[command(about = "Mark task as finished.")]
pub struct FinishArgs {
    #[arg(short, long, help = "Task number.")]
    pub id: i32,
}

impl FinishArgs {
    pub fn execute(&self) -> Result<String, Box<dyn std::error::Error>> {
        let result = format!("FinishCommand wurde für ID: {} aufgerufen.", self.id);
        // Implement me!
        Ok(result)
    }
}
