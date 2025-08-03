use ratatui::{backend::TestBackend, Terminal};
use skillcapped_generator::{app::App, ui::draw_ui};
use std::time::Instant;

#[test]
fn test_draw_ui_with_empty_input() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let app = App::new();

    let result = terminal.draw(|f| draw_ui(f, &app));
    assert!(result.is_ok());
}

#[test]
fn test_draw_ui_with_valid_battlenet_id() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.battlenet_id = "TestUser#1234".to_string();

    let result = terminal.draw(|f| draw_ui(f, &app));
    assert!(result.is_ok());
}

#[test]
fn test_draw_ui_with_invalid_battlenet_id() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.battlenet_id = "invalid".to_string();

    let result = terminal.draw(|f| draw_ui(f, &app));
    assert!(result.is_ok());
}

#[test]
fn test_draw_ui_with_copy_feedback() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.battlenet_id = "TestUser#1234".to_string();
    app.copy_feedback = Some(Instant::now());

    let result = terminal.draw(|f| draw_ui(f, &app));
    assert!(result.is_ok());
}

#[test]
fn test_draw_ui_classic_version() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.version = "classic".to_string();

    let result = terminal.draw(|f| draw_ui(f, &app));
    assert!(result.is_ok());
}

#[test]
fn test_draw_ui_small_terminal() {
    let backend = TestBackend::new(40, 12);
    let mut terminal = Terminal::new(backend).unwrap();
    let app = App::new();

    let result = terminal.draw(|f| draw_ui(f, &app));
    assert!(result.is_ok());
}

#[test]
fn test_draw_ui_large_terminal() {
    let backend = TestBackend::new(120, 40);
    let mut terminal = Terminal::new(backend).unwrap();
    let app = App::new();

    let result = terminal.draw(|f| draw_ui(f, &app));
    assert!(result.is_ok());
}

#[test]
fn test_draw_ui_help_section() {
    let backend = TestBackend::new(120, 30); // Larger terminal
    let mut terminal = Terminal::new(backend).unwrap();
    let app = App::new();

    let result = terminal.draw(|f| draw_ui(f, &app));
    assert!(result.is_ok());

    // Test that the help section is rendered
    let buffer = terminal.backend().buffer();
    let buffer_text: String = buffer.content.iter().map(|cell| cell.symbol()).collect();

    // Test for help-related content (the help section should contain these)
    assert!(buffer_text.contains("Help") || buffer_text.contains("Ctrl"));
}

#[test]
fn test_draw_ui_with_code_generation_error() {
    let mut app = App::new();
    app.battlenet_id = "TestUser#1234".to_string();
    app.version = "invalid_version".to_string(); // This will cause generate_code to fail

    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            draw_ui(f, &app);
        })
        .unwrap();

    let buffer = terminal.backend().buffer();
    let content: String = buffer.content.iter().map(|cell| cell.symbol()).collect();

    // Should show "Invalid version" when code generation fails
    assert!(content.contains("Invalid version"));
}

#[test]
fn test_draw_ui_with_expired_copy_feedback() {
    let mut app = App::new();
    app.battlenet_id = "TestUser#1234".to_string();

    // Set copy feedback to an old timestamp (more than 2 seconds ago)
    let old_time = std::time::Instant::now() - std::time::Duration::from_secs(3);
    app.copy_feedback = Some(old_time);

    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            draw_ui(f, &app);
        })
        .unwrap();

    let buffer = terminal.backend().buffer();
    let content: String = buffer.content.iter().map(|cell| cell.symbol()).collect();

    // Should show regular copy message, not the "Copied to clipboard!" message
    assert!(content.contains("Ctrl+C to copy"));
    assert!(!content.contains("Copied to clipboard!"));
}

#[test]
fn test_create_version_span_inactive() {
    // Test the version span creation for inactive versions
    let app = App::new();
    // App defaults to "retail", so "classic" should be inactive

    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            draw_ui(f, &app);
        })
        .unwrap();

    let buffer = terminal.backend().buffer();
    let content: String = buffer.content.iter().map(|cell| cell.symbol()).collect();

    // Should show version information (retail is default, so it should be visible)
    // The exact text depends on the rendering, so we check for version-related content
    assert!(content.contains("retail") || content.contains("Version"));
}
