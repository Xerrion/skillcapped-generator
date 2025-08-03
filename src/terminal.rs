use crossterm::{
    event::{self, Event, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
};
use std::{
    io,
    time::{Duration, Instant},
};

use crate::{app::App, input::handle_key_event, ui::draw_ui};

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut app = App::new();
    app.reset_input();

    loop {
        // Sanitize input before displaying
        app.sanitize_input();

        // Draw the UI
        terminal.draw(|f| draw_ui(f, &app))?;

        // Handle events
        if event::poll(Duration::from_millis(200))? {
            match event::read()? {
                Event::Key(key) => {
                    // Only handle key press events
                    if key.kind != KeyEventKind::Press {
                        continue;
                    }

                    if handle_key_event(&mut app, key) {
                        return Ok(()); // Quit signal received
                    }
                }
                Event::Mouse(_) => {
                    // Ignore mouse events
                }
                _ => {}
            }
        }

        // Update copy feedback
        update_copy_feedback(&mut app);
    }
}

pub fn setup_terminal()
-> Result<Terminal<CrosstermBackend<std::io::Stdout>>, Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    Ok(Terminal::new(backend)?)
}

pub fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

pub fn update_copy_feedback(app: &mut App) {
    if let Some(copy_instant) = app.copy_feedback {
        if Instant::now().duration_since(copy_instant).as_secs() >= 1 {
            app.copy_feedback = None;
        }
    }
}
