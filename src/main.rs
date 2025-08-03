pub mod app;
pub mod ui;
pub mod input;
pub mod terminal;

use terminal::{setup_terminal, restore_terminal, run_app};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = setup_terminal()?;
    let res = run_app(&mut terminal);
    restore_terminal(&mut terminal)?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}