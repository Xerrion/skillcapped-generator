use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::time::Instant;
use arboard::Clipboard;
use crate::app::App;

#[cfg(test)]
mod tests;

pub fn handle_key_event(app: &mut App, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Char(c) if !has_modifiers(&key) => {
            app.add_char(c);
        }
        KeyCode::Backspace => {
            app.remove_char();
        }
        KeyCode::Tab => {
            app.toggle_version();
        }
        KeyCode::Enter => {
            app.use_lowercase = !app.use_lowercase;
        }
        KeyCode::Esc => {
            app.reset_input();
        }
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            handle_copy_code(app);
        }
        KeyCode::Char('g') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            open_github_link();
        }
        KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            return true; // Signal to quit
        }
        _ => {}
    }
    false // Continue running
}

fn has_modifiers(key: &KeyEvent) -> bool {
    key.modifiers.contains(KeyModifiers::CONTROL) 
        || key.modifiers.contains(KeyModifiers::ALT) 
        || key.modifiers.contains(KeyModifiers::SUPER)
}

fn handle_copy_code(app: &mut App) {
    if app.is_valid_battlenet_id() {
        if let Ok(unlock_code) = app.generate_code() {
            if let Ok(mut clipboard) = Clipboard::new() {
                if clipboard.set_text(unlock_code).is_ok() {
                    app.copy_feedback = Some(Instant::now());
                }
            }
        }
    }
}

fn open_github_link() {
    let url = "https://github.com/Xerrion";
    
    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("cmd")
            .args(["/c", "start", url])
            .spawn();
    }
    
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open")
            .arg(url)
            .spawn();
    }
    
    #[cfg(target_os = "linux")]
    {
        let _ = std::process::Command::new("xdg-open")
            .arg(url)
            .spawn();
    }
}
