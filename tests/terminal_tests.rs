use ratatui::{backend::CrosstermBackend, backend::TestBackend, Terminal};
use skillcapped_generator::{
    app::App,
    terminal::{restore_terminal, run_app, setup_terminal, update_copy_feedback},
};
use std::time::{Duration, Instant};

#[test]
fn test_setup_and_restore_terminal() {
    // Note: This test might fail in CI environments without a proper terminal
    // But it's useful for local testing

    // We can't easily test the actual terminal setup/restore in a unit test
    // because it requires a real terminal, but we can test that the functions exist
    // and have the correct signatures

    // This is more of a compilation test
    let _setup_fn: fn() -> Result<
        Terminal<CrosstermBackend<std::io::Stdout>>,
        Box<dyn std::error::Error>,
    > = setup_terminal;
    let _restore_fn: fn(
        &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    ) -> Result<(), Box<dyn std::error::Error>> = restore_terminal;
}

#[test]
fn test_update_copy_feedback() {
    let mut app = App::new();

    // Test with no copy feedback
    update_copy_feedback(&mut app);
    assert!(app.copy_feedback.is_none());

    // Test with recent copy feedback (should remain)
    app.copy_feedback = Some(Instant::now());
    update_copy_feedback(&mut app);
    assert!(app.copy_feedback.is_some());

    // Test with old copy feedback (should be cleared)
    app.copy_feedback = Some(Instant::now() - Duration::from_secs(2));
    update_copy_feedback(&mut app);
    assert!(app.copy_feedback.is_none());
}

#[test]
fn test_run_app_with_test_backend() {
    // We can't easily test the full run_app function because it's an infinite loop
    // that requires user input, but we can test that it compiles and the types are correct

    let backend = TestBackend::new(80, 24);
    let _terminal = Terminal::new(backend).unwrap();

    // We can't actually run this because it would block forever waiting for input
    // but we can ensure the function signature is correct
    let _run_fn: fn(&mut Terminal<TestBackend>) -> std::io::Result<()> = run_app;
}
