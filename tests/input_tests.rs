use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use skillcapped_generator::{app::App, input::handle_key_event};

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

#[test]
fn test_handle_ctrl_g_github() {
    let mut app = App::new();
    let original_id = app.battlenet_id.clone();
    let key = create_key_event(KeyCode::Char('g'), KeyModifiers::CONTROL);

    let should_quit = handle_key_event(&mut app, key);
    assert!(!should_quit);
    // GitHub link opening doesn't change app state
    assert_eq!(app.battlenet_id, original_id);
}

#[test]
fn test_has_modifiers_comprehensive() {
    // Test CONTROL modifier
    let key = create_key_event(KeyCode::Char('a'), KeyModifiers::CONTROL);
    let should_quit = handle_key_event(&mut App::new(), key);
    assert!(!should_quit);

    // Test ALT modifier
    let key = create_key_event(KeyCode::Char('b'), KeyModifiers::ALT);
    let should_quit = handle_key_event(&mut App::new(), key);
    assert!(!should_quit);

    // Test SUPER modifier
    let key = create_key_event(KeyCode::Char('c'), KeyModifiers::SUPER);
    let should_quit = handle_key_event(&mut App::new(), key);
    assert!(!should_quit);

    // Test SHIFT modifier (should not block character input)
    let mut app = App::new();
    let key = create_key_event(KeyCode::Char('d'), KeyModifiers::SHIFT);
    let should_quit = handle_key_event(&mut app, key);
    assert!(!should_quit);
    assert_eq!(app.battlenet_id, "d"); // SHIFT should not block regular chars
}

#[test]
fn test_handle_copy_code_invalid_id() {
    let mut app = App::new();
    app.battlenet_id = "invalid".to_string(); // Invalid Battle.net ID
    let key = create_key_event(KeyCode::Char('c'), KeyModifiers::CONTROL);

    let initial_feedback = app.copy_feedback;
    let should_quit = handle_key_event(&mut app, key);
    assert!(!should_quit);
    // Copy feedback should not be set for invalid ID
    assert_eq!(app.copy_feedback, initial_feedback);
}

#[test]
fn test_handle_copy_code_valid_id() {
    let mut app = App::new();
    app.battlenet_id = "TestUser#1234".to_string(); // Valid Battle.net ID
    let key = create_key_event(KeyCode::Char('c'), KeyModifiers::CONTROL);

    let should_quit = handle_key_event(&mut app, key);
    assert!(!should_quit);
    // Note: copy_feedback might be set if clipboard is available
    // We can't guarantee clipboard availability in tests, so we just ensure no panic
}

#[test]
fn test_special_key_combinations() {
    let mut app = App::new();
    
    // Test Ctrl+Shift+C (should not trigger copy)
    let key = create_key_event(
        KeyCode::Char('c'), 
        KeyModifiers::CONTROL | KeyModifiers::SHIFT
    );
    let should_quit = handle_key_event(&mut app, key);
    assert!(!should_quit);
    
    // Test Ctrl+Alt+V (should not trigger paste)
    let key = create_key_event(
        KeyCode::Char('v'), 
        KeyModifiers::CONTROL | KeyModifiers::ALT
    );
    let should_quit = handle_key_event(&mut app, key);
    assert!(!should_quit);
}

#[test]
fn test_paste_functionality_behavior() {
    let mut app = App::new();
    app.battlenet_id = "ExistingData".to_string();
    app.use_lowercase = true;
    app.version = "classic".to_string();
    
    let key = create_key_event(KeyCode::Char('v'), KeyModifiers::CONTROL);
    let should_quit = handle_key_event(&mut app, key);
    
    assert!(!should_quit);
    // Paste should reset input (clear existing data)
    // but preserve other settings like use_lowercase and version
    assert!(app.use_lowercase);
    assert_eq!(app.version, "classic");
}

#[test]
fn test_various_key_codes() {
    let mut app = App::new();
    
    // Test other key codes that should be ignored
    let keys_to_ignore = vec![
        KeyCode::Home,
        KeyCode::End,
        KeyCode::PageUp,
        KeyCode::PageDown,
        KeyCode::Insert,
        KeyCode::Delete,
        KeyCode::F(1),
        KeyCode::Up,
        KeyCode::Down,
        KeyCode::Left,
        KeyCode::Right,
    ];
    
    for key_code in keys_to_ignore {
        let key = create_key_event(key_code, KeyModifiers::empty());
        let should_quit = handle_key_event(&mut app, key);
        assert!(!should_quit);
        // App state should remain unchanged
        assert_eq!(app.battlenet_id, "");
    }
}

#[test]
fn test_character_input_comprehensive() {
    let mut app = App::new();
    
    // Test various characters
    let chars = vec!['a', 'Z', '1', '#', '_', '-', ' '];
    
    for c in chars {
        app.reset_input();
        let key = create_key_event(KeyCode::Char(c), KeyModifiers::empty());
        let should_quit = handle_key_event(&mut app, key);
        assert!(!should_quit);
        assert_eq!(app.battlenet_id, c.to_string());
    }
}

#[test]
fn test_multiple_key_sequence() {
    let mut app = App::new();
    
    // Simulate typing a Battle.net ID
    let sequence = "TestUser#1234";
    for c in sequence.chars() {
        let key = create_key_event(KeyCode::Char(c), KeyModifiers::empty());
        let should_quit = handle_key_event(&mut app, key);
        assert!(!should_quit);
    }
    
    assert_eq!(app.battlenet_id, sequence);
    
    // Test backspace sequence
    for _ in 0..4 {
        let key = create_key_event(KeyCode::Backspace, KeyModifiers::empty());
        let should_quit = handle_key_event(&mut app, key);
        assert!(!should_quit);
    }
    
    assert_eq!(app.battlenet_id, "TestUser#");
}

#[test]
fn test_reset_and_toggle_combinations() {
    let mut app = App::new();
    app.battlenet_id = "Test#1234".to_string();
    app.use_lowercase = false;
    app.version = "retail".to_string();
    
    // Test escape reset
    let key = create_key_event(KeyCode::Esc, KeyModifiers::empty());
    let should_quit = handle_key_event(&mut app, key);
    assert!(!should_quit);
    assert_eq!(app.battlenet_id, "");
    
    // Test enter toggle
    let key = create_key_event(KeyCode::Enter, KeyModifiers::empty());
    let should_quit = handle_key_event(&mut app, key);
    assert!(!should_quit);
    assert!(app.use_lowercase);
    
    // Test another enter toggle
    let should_quit = handle_key_event(&mut app, key);
    assert!(!should_quit);
    assert!(!app.use_lowercase);
    
    // Test tab version toggle
    let key = create_key_event(KeyCode::Tab, KeyModifiers::empty());
    let should_quit = handle_key_event(&mut app, key);
    assert!(!should_quit);
    assert_eq!(app.version, "classic");
}
