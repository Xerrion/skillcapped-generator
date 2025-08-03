use ratatui::{Terminal, backend::TestBackend};
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
