pub mod app;
pub mod input;
pub mod terminal;
pub mod ui;

use terminal::{restore_terminal, run_app, setup_terminal};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = setup_terminal()?;
    let res = run_app(&mut terminal);
    restore_terminal(&mut terminal)?;

    if let Err(err) = res {
        println!("Error: {err:?}");
    }

    Ok(())
}
