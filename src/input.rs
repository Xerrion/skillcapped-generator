use crate::app::App;
use arboard::Clipboard;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::time::Instant;

pub fn handle_key_event(app: &mut App, key: KeyEvent) -> bool {
    if handle_control_keys(app, &key) {
        return true; // Quit signal
    }

    handle_regular_keys(app, &key);
    false // Continue running
}

fn handle_control_keys(app: &mut App, key: &KeyEvent) -> bool {
    if !key.modifiers.contains(KeyModifiers::CONTROL) {
        return false;
    }

    match key.code {
        KeyCode::Char('c') => {
            handle_copy_code(app);
            false
        }
        KeyCode::Char('v') => {
            handle_paste_battlenet_id(app);
            false
        }
        KeyCode::Char('g') => {
            open_github_link();
            false
        }
        KeyCode::Char('q') => true, // Signal to quit
        _ => false,
    }
}

fn handle_regular_keys(app: &mut App, key: &KeyEvent) {
    match key.code {
        KeyCode::Char(c) if !has_modifiers(key) => app.add_char(c),
        KeyCode::Backspace => app.remove_char(),
        KeyCode::Tab => app.toggle_version(),
        KeyCode::Enter => app.use_lowercase = !app.use_lowercase,
        KeyCode::Esc => app.reset_input(),
        _ => {}
    }
}

fn has_modifiers(key: &KeyEvent) -> bool {
    key.modifiers.contains(KeyModifiers::CONTROL)
        || key.modifiers.contains(KeyModifiers::ALT)
        || key.modifiers.contains(KeyModifiers::SUPER)
}

fn handle_copy_code(app: &mut App) {
    if !app.is_valid_battlenet_id() {
        return;
    }

    let Ok(unlock_code) = app.generate_code() else {
        return;
    };

    let Ok(mut clipboard) = Clipboard::new() else {
        return;
    };

    if clipboard.set_text(unlock_code).is_ok() {
        app.copy_feedback = Some(Instant::now());
    }
}

fn handle_paste_battlenet_id(app: &mut App) {
    let Ok(mut clipboard) = Clipboard::new() else {
        return;
    };

    let Ok(clipboard_text) = clipboard.get_text() else {
        return;
    };

    // Clear current input and set to clipboard content
    app.reset_input();

    // Add each character from clipboard
    for c in clipboard_text.chars() {
        app.add_char(c);
    }

    // Sanitize the pasted input
    app.sanitize_input();
}

fn open_github_link() {
    let url = "https://github.com/Xerrion";

    let command_result = get_platform_open_command(url);
    if let Some((program, args)) = command_result {
        let _ = std::process::Command::new(program).args(args).spawn();
    }
}

fn get_platform_open_command(url: &str) -> Option<(&'static str, Vec<&str>)> {
    match std::env::consts::OS {
        "windows" => Some(("cmd", vec!["/c", "start", url])),
        "macos" => Some(("open", vec![url])),
        "linux" => Some(("xdg-open", vec![url])),
        _ => None,
    }
}
