use skillcapped_generator::{app::App, input::handle_key_event};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, KeyEventKind, KeyEventState};

fn create_key_event(code: KeyCode, modifiers: KeyModifiers) -> KeyEvent {
    KeyEvent {
        code,
        modifiers,
        kind: KeyEventKind::Press,
        state: KeyEventState::empty(),
    }
}

#[test]
fn test_handle_regular_char() {
    let mut app = App::new();
    let key = create_key_event(KeyCode::Char('A'), KeyModifiers::empty());
    
    let should_quit = handle_key_event(&mut app, key);
    assert!(!should_quit);
    assert_eq!(app.battlenet_id, "A");
}

#[test]
fn test_handle_backspace() {
    let mut app = App::new();
    app.battlenet_id = "Test".to_string();
    let key = create_key_event(KeyCode::Backspace, KeyModifiers::empty());
    
    let should_quit = handle_key_event(&mut app, key);
    assert!(!should_quit);
    assert_eq!(app.battlenet_id, "Tes");
}

#[test]
fn test_handle_tab_toggle_version() {
    let mut app = App::new();
    app.version = "retail".to_string();
    let key = create_key_event(KeyCode::Tab, KeyModifiers::empty());
    
    let should_quit = handle_key_event(&mut app, key);
    assert!(!should_quit);
    assert_eq!(app.version, "classic");
    
    let should_quit = handle_key_event(&mut app, key);
    assert!(!should_quit);
    assert_eq!(app.version, "retail");
}

#[test]
fn test_handle_enter_toggle_lowercase() {
    let mut app = App::new();
    assert!(!app.use_lowercase);
    
    let key = create_key_event(KeyCode::Enter, KeyModifiers::empty());
    let should_quit = handle_key_event(&mut app, key);
    assert!(!should_quit);
    assert!(app.use_lowercase);
    
    let should_quit = handle_key_event(&mut app, key);
    assert!(!should_quit);
    assert!(!app.use_lowercase);
}

#[test]
fn test_handle_escape_reset() {
    let mut app = App::new();
    app.battlenet_id = "TestUser#1234".to_string();
    let key = create_key_event(KeyCode::Esc, KeyModifiers::empty());
    
    let should_quit = handle_key_event(&mut app, key);
    assert!(!should_quit);
    assert_eq!(app.battlenet_id, "");
}

#[test]
fn test_handle_ctrl_q_quit() {
    let mut app = App::new();
    let key = create_key_event(KeyCode::Char('q'), KeyModifiers::CONTROL);
    
    let should_quit = handle_key_event(&mut app, key);
    assert!(should_quit);
}

#[test]
fn test_handle_ctrl_c_copy_invalid_id() {
    let mut app = App::new();
    app.battlenet_id = "invalid".to_string(); // Invalid Battle.net ID
    let key = create_key_event(KeyCode::Char('c'), KeyModifiers::CONTROL);
    
    let should_quit = handle_key_event(&mut app, key);
    assert!(!should_quit);
    // Should not set copy_feedback for invalid ID
    assert!(app.copy_feedback.is_none());
}

#[test]
fn test_handle_ctrl_c_copy_valid_id() {
    let mut app = App::new();
    app.battlenet_id = "TestUser#1234".to_string(); // Valid Battle.net ID
    let key = create_key_event(KeyCode::Char('c'), KeyModifiers::CONTROL);
    
    let should_quit = handle_key_event(&mut app, key);
    assert!(!should_quit);
    // Note: copy_feedback might be set if clipboard is available
}

#[test]
fn test_ignore_char_with_modifiers() {
    let mut app = App::new();
    let key = create_key_event(KeyCode::Char('a'), KeyModifiers::CONTROL);
    
    let should_quit = handle_key_event(&mut app, key);
    assert!(!should_quit);
    assert_eq!(app.battlenet_id, ""); // Should not add the character
}

#[test]
fn test_ignore_char_with_alt() {
    let mut app = App::new();
    let key = create_key_event(KeyCode::Char('a'), KeyModifiers::ALT);
    
    let should_quit = handle_key_event(&mut app, key);
    assert!(!should_quit);
    assert_eq!(app.battlenet_id, ""); // Should not add the character
}

#[test]
fn test_ignore_unknown_key() {
    let mut app = App::new();
    let key = create_key_event(KeyCode::Home, KeyModifiers::empty());
    
    let should_quit = handle_key_event(&mut app, key);
    assert!(!should_quit);
    // App state should remain unchanged
    assert_eq!(app.battlenet_id, "");
    assert_eq!(app.version, "retail");
    assert!(!app.use_lowercase);
}

#[test]
fn test_handle_ctrl_v_paste() {
    let mut app = App::new();
    app.battlenet_id = "OldData".to_string();
    let key = create_key_event(KeyCode::Char('v'), KeyModifiers::CONTROL);
    
    let should_quit = handle_key_event(&mut app, key);
    assert!(!should_quit);
    // We can't easily test actual clipboard functionality in unit tests,
    // but we can verify the handler doesn't panic and processes the input
    // The battlenet_id might change if clipboard is available, or stay the same if not
}
