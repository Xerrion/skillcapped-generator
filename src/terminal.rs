use crossterm::{
    event::{self, Event, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
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
        if handle_app_iteration(terminal, &mut app)? {
            return Ok(());
        }

        update_copy_feedback(&mut app);
    }
}

fn handle_app_iteration<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    // Sanitize input before displaying
    app.sanitize_input();

    // Draw the UI
    terminal.draw(|f| draw_ui(f, app))?;

    // Handle events
    if event::poll(Duration::from_millis(200))? {
        if let Some(should_quit) = handle_event(app)? {
            return Ok(should_quit);
        }
    }

    Ok(false)
}

fn handle_event(app: &mut App) -> io::Result<Option<bool>> {
    let event = event::read()?;

    match event {
        Event::Key(key) if key.kind == KeyEventKind::Press => Ok(Some(handle_key_event(app, key))),
        Event::Mouse(_) => Ok(None), // Ignore mouse events
        _ => Ok(None),
    }
}

pub fn setup_terminal(
) -> Result<Terminal<CrosstermBackend<std::io::Stdout>>, Box<dyn std::error::Error>> {
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
    app.copy_feedback = app
        .copy_feedback
        .filter(|&copy_instant| Instant::now().duration_since(copy_instant).as_secs() < 1);
}
