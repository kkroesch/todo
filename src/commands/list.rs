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
#[command(about = "List unfinished task items.")]
pub struct ListArgs {
    #[arg(short = 'a', long = "all", help = "List finished tasks, too.")]
    pub all: bool,
}

impl ListArgs {
    pub fn execute(&self) -> Result<String, Box<dyn std::error::Error>> {
        let result = format!("ListCommand wurde aufgerufen.");
        // Implement me!
        Ok(result)
    }
}
