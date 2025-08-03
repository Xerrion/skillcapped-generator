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
fn test_handle_ctrl_g_github_integration() {
    let mut app = App::new();
    let original_state = (
        app.battlenet_id.clone(),
        app.use_lowercase,
        app.version.clone(),
    );

    let key = create_key_event(KeyCode::Char('g'), KeyModifiers::CONTROL);
    let should_quit = handle_key_event(&mut app, key);

    assert!(!should_quit);

    // Verify that opening GitHub link doesn't change the app state
    assert_eq!(
        (app.battlenet_id, app.use_lowercase, app.version),
        original_state
    );
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
    let should_quit = handle_key_event(&mut app, key);

    assert!(!should_quit);
    // Should not crash when trying to copy invalid ID
    assert!(app.copy_feedback.is_none()); // No feedback for invalid ID
}

#[test]
fn test_handle_copy_code_generation_error() {
    let mut app = App::new();
    app.battlenet_id = "TestUser#1234".to_string();
    app.version = "invalid_version".to_string(); // This will cause generate_code to fail

    let key = create_key_event(KeyCode::Char('c'), KeyModifiers::CONTROL);
    let should_quit = handle_key_event(&mut app, key);

    assert!(!should_quit);
    // Should not crash when code generation fails
    assert!(app.copy_feedback.is_none()); // No feedback when generation fails
}

#[test]
fn test_handle_paste_empty_clipboard() {
    let mut app = App::new();
    app.battlenet_id = "existing".to_string();

    // This test checks the clipboard error handling path
    let key = create_key_event(KeyCode::Char('v'), KeyModifiers::CONTROL);
    let should_quit = handle_key_event(&mut app, key);

    assert!(!should_quit);
    // The function should handle clipboard errors gracefully
    // Note: In real scenario, this depends on clipboard state
}

#[test]
fn test_handle_paste_with_special_characters() {
    let mut app = App::new();

    // Simulate pasting text with special characters that need sanitization
    // Since we can't control the actual clipboard in tests, we test the sanitization logic
    app.battlenet_id = "Test@User#1234!".to_string();
    app.sanitize_input();

    assert_eq!(app.battlenet_id, "TestUser#1234");
}

#[test]
fn test_has_modifiers_edge_cases() {
    // Test ALT modifier
    let key_alt = create_key_event(KeyCode::Char('a'), KeyModifiers::ALT);
    let has_alt_mod = key_alt.modifiers.contains(KeyModifiers::CONTROL)
        || key_alt.modifiers.contains(KeyModifiers::ALT)
        || key_alt.modifiers.contains(KeyModifiers::SUPER);
    assert!(has_alt_mod);

    // Test SUPER modifier
    let key_super = create_key_event(KeyCode::Char('a'), KeyModifiers::SUPER);
    let has_super_mod = key_super.modifiers.contains(KeyModifiers::CONTROL)
        || key_super.modifiers.contains(KeyModifiers::ALT)
        || key_super.modifiers.contains(KeyModifiers::SUPER);
    assert!(has_super_mod);

    // Test no modifiers
    let key_none = create_key_event(KeyCode::Char('a'), KeyModifiers::empty());
    let has_no_mod = key_none.modifiers.contains(KeyModifiers::CONTROL)
        || key_none.modifiers.contains(KeyModifiers::ALT)
        || key_none.modifiers.contains(KeyModifiers::SUPER);
    assert!(!has_no_mod);
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

#[test]
fn test_handle_ctrl_c_with_invalid_version() {
    let mut app = App::new();
    app.battlenet_id = "TestUser#1234".to_string();
    app.version = "invalid_version".to_string(); // This will cause generate_code to fail

    let key = create_key_event(KeyCode::Char('c'), KeyModifiers::CONTROL);
    let should_quit = handle_key_event(&mut app, key);
    assert!(!should_quit);
    // The copy_feedback should not be set if generation fails
    assert!(app.copy_feedback.is_none());
}

#[test]
fn test_generate_code_with_invalid_version() {
    let mut app = App::new();
    app.battlenet_id = "TestUser#1234".to_string();
    app.version = "invalid_version".to_string();

    let result = app.generate_code();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid version");
}

// Test to ensure clipboard errors are handled gracefully
// Note: These tests may not hit the exact error paths due to clipboard system dependencies
#[test]
fn test_paste_battlenet_id_error_handling() {
    // This test mainly verifies the function can be called without panicking
    // The actual clipboard error paths are hard to test reliably in a unit test
    let mut app = App::new();
    let key = create_key_event(KeyCode::Char('v'), KeyModifiers::CONTROL);

    let should_quit = handle_key_event(&mut app, key);
    assert!(!should_quit);
    // The function should complete without crashing regardless of clipboard state
}
