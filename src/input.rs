use crate::app::App;
use arboard::Clipboard;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::time::Instant;

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
        KeyCode::Char('v') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            handle_paste_battlenet_id(app);
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

    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("cmd")
            .args(["/c", "start", url])
            .spawn();
    }

    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open").arg(url).spawn();
    }

    #[cfg(target_os = "linux")]
    {
        let _ = std::process::Command::new("xdg-open").arg(url).spawn();
    }
}
