use base64::{Engine as _, engine::general_purpose};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use ratatui::{Terminal, backend::TestBackend};
use skillcapped_generator::{app::App, input::handle_key_event, ui::draw_ui};

fn create_key_event(code: KeyCode, modifiers: KeyModifiers) -> KeyEvent {
    KeyEvent {
        code,
        modifiers,
        kind: KeyEventKind::Press,
        state: KeyEventState::empty(),
    }
}

#[test]
fn test_full_workflow() {
    let mut app = App::new();

    // Simulate typing a Battle.net ID
    let battlenet_id = "TestUser#1234";
    for c in battlenet_id.chars() {
        let key = create_key_event(KeyCode::Char(c), KeyModifiers::empty());
        handle_key_event(&mut app, key);
    }

    assert_eq!(app.battlenet_id, battlenet_id);
    assert!(app.is_valid_battlenet_id());

    // Generate code
    let code = app.generate_code().unwrap();
    assert!(!code.is_empty());

    // Validate the generated code
    assert!(app.validate_code(&code));

    // Test UI rendering
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let result = terminal.draw(|f| draw_ui(f, &app));
    assert!(result.is_ok());
}

#[test]
fn test_version_switching_workflow() {
    let mut app = App::new();
    app.battlenet_id = "TestUser#1234".to_string();

    // Start with retail
    assert_eq!(app.version, "retail");
    let retail_code = app.generate_code().unwrap();

    // Switch to classic
    let tab_key = create_key_event(KeyCode::Tab, KeyModifiers::empty());
    handle_key_event(&mut app, tab_key);
    assert_eq!(app.version, "classic");
    let classic_code = app.generate_code().unwrap();

    // Both should generate valid codes
    assert!(!retail_code.is_empty());
    assert!(!classic_code.is_empty());

    // In this implementation, both versions use the same config, so codes should be the same
    assert_eq!(retail_code, classic_code);
}

#[test]
fn test_case_sensitivity_workflow() {
    let mut app = App::new();
    app.battlenet_id = "TestUser#1234".to_string();

    // Generate code without lowercase
    let normal_code = app.generate_code().unwrap();

    // Toggle lowercase
    let enter_key = create_key_event(KeyCode::Enter, KeyModifiers::empty());
    handle_key_event(&mut app, enter_key);
    assert!(app.use_lowercase);

    // Generate code with lowercase
    let lowercase_code = app.generate_code().unwrap();

    // Codes should be different
    assert_ne!(normal_code, lowercase_code);

    // Both should be valid base64
    assert!(general_purpose::STANDARD.decode(&normal_code).is_ok());
    assert!(general_purpose::STANDARD.decode(&lowercase_code).is_ok());
}

#[test]
fn test_input_sanitization_workflow() {
    let mut app = App::new();

    // Type invalid characters
    let invalid_input = "Test@User!#12$34";
    for c in invalid_input.chars() {
        let key = create_key_event(KeyCode::Char(c), KeyModifiers::empty());
        handle_key_event(&mut app, key);
    }

    // Sanitize input
    app.sanitize_input();

    // Should only keep alphanumeric and #
    assert_eq!(app.battlenet_id, "TestUser#1234");
    assert!(app.is_valid_battlenet_id());
}

#[test]
fn test_backspace_workflow() {
    let mut app = App::new();

    // Type some text
    let text = "TestUser#1234";
    for c in text.chars() {
        let key = create_key_event(KeyCode::Char(c), KeyModifiers::empty());
        handle_key_event(&mut app, key);
    }

    // Backspace a few characters
    let backspace_key = create_key_event(KeyCode::Backspace, KeyModifiers::empty());
    handle_key_event(&mut app, backspace_key);
    handle_key_event(&mut app, backspace_key);
    handle_key_event(&mut app, backspace_key);
    handle_key_event(&mut app, backspace_key);

    assert_eq!(app.battlenet_id, "TestUser#");
    assert!(!app.is_valid_battlenet_id()); // Should be invalid now (no digits after #)
}

#[test]
fn test_reset_workflow() {
    let mut app = App::new();

    // Set up some state
    app.battlenet_id = "TestUser#1234".to_string();
    app.use_lowercase = true;
    app.version = "classic".to_string();

    // Reset input
    let esc_key = create_key_event(KeyCode::Esc, KeyModifiers::empty());
    handle_key_event(&mut app, esc_key);

    // Only battlenet_id should be reset
    assert_eq!(app.battlenet_id, "");
    assert!(app.use_lowercase); // Should remain true
    assert_eq!(app.version, "classic"); // Should remain classic
}

#[test]
fn test_quit_workflow() {
    let mut app = App::new();

    // Simulate Ctrl+Q
    let quit_key = create_key_event(KeyCode::Char('q'), KeyModifiers::CONTROL);
    let should_quit = handle_key_event(&mut app, quit_key);

    assert!(should_quit);
}

#[test]
fn test_ui_with_different_states() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    // Test empty state
    let app = App::new();
    assert!(terminal.draw(|f| draw_ui(f, &app)).is_ok());

    // Test with invalid ID
    let mut app = App::new();
    app.battlenet_id = "invalid".to_string();
    assert!(terminal.draw(|f| draw_ui(f, &app)).is_ok());

    // Test with valid ID
    let mut app = App::new();
    app.battlenet_id = "TestUser#1234".to_string();
    assert!(terminal.draw(|f| draw_ui(f, &app)).is_ok());

    // Test with classic version
    let mut app = App::new();
    app.version = "classic".to_string();
    app.battlenet_id = "TestUser#1234".to_string();
    assert!(terminal.draw(|f| draw_ui(f, &app)).is_ok());
}
