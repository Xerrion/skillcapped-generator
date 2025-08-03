use skillcapped_generator::app::App;

#[test]
fn test_new_app() {
    let app = App::new();
    assert_eq!(app.battlenet_id, "");
    assert!(!app.use_lowercase);
    assert_eq!(app.version, "retail");
    assert!(app.copy_feedback.is_none());
}

#[test]
fn test_reset_input() {
    let mut app = App::new();
    app.battlenet_id = "TestUser#1234".to_string();
    app.reset_input();
    assert_eq!(app.battlenet_id, "");
}

#[test]
fn test_toggle_version() {
    let mut app = App::new();
    assert_eq!(app.version, "retail");

    app.toggle_version();
    assert_eq!(app.version, "classic");

    app.toggle_version();
    assert_eq!(app.version, "retail");
}

#[test]
fn test_add_char() {
    let mut app = App::new();
    app.add_char('T');
    app.add_char('e');
    app.add_char('s');
    app.add_char('t');
    assert_eq!(app.battlenet_id, "Test");
}

#[test]
fn test_remove_char() {
    let mut app = App::new();
    app.battlenet_id = "Test".to_string();
    app.remove_char();
    assert_eq!(app.battlenet_id, "Tes");

    app.remove_char();
    assert_eq!(app.battlenet_id, "Te");
}

#[test]
fn test_sanitize_input() {
    let mut app = App::new();
    app.battlenet_id = "Test@User#1234!".to_string();
    app.sanitize_input();
    assert_eq!(app.battlenet_id, "TestUser#1234");
}

#[test]
fn test_is_valid_battlenet_id() {
    let mut app = App::new();

    // Valid cases
    app.battlenet_id = "TestUser#1234".to_string();
    assert!(app.is_valid_battlenet_id());

    app.battlenet_id = "Xerrion#2624".to_string();
    assert!(app.is_valid_battlenet_id());

    app.battlenet_id = "User123#567890".to_string();
    assert!(app.is_valid_battlenet_id());

    // Invalid cases
    app.battlenet_id = "".to_string();
    assert!(!app.is_valid_battlenet_id());

    app.battlenet_id = "TestUser".to_string();
    assert!(!app.is_valid_battlenet_id());

    app.battlenet_id = "#1234".to_string();
    assert!(!app.is_valid_battlenet_id());

    app.battlenet_id = "TestUser#".to_string();
    assert!(!app.is_valid_battlenet_id());

    app.battlenet_id = "TestUser#123".to_string(); // Less than 4 digits
    assert!(!app.is_valid_battlenet_id());

    app.battlenet_id = "Test@User#1234".to_string(); // Invalid character in name
    assert!(!app.is_valid_battlenet_id());

    app.battlenet_id = "TestUser#12a4".to_string(); // Non-digit in number
    assert!(!app.is_valid_battlenet_id());

    app.battlenet_id = "Test#User#1234".to_string(); // Multiple # characters
    assert!(!app.is_valid_battlenet_id());
}

#[test]
fn test_generate_code() {
    let mut app = App::new();
    app.battlenet_id = "TestUser#1234".to_string();

    // Test retail version
    app.version = "retail".to_string();
    let result = app.generate_code();
    assert!(result.is_ok());
    let code = result.unwrap();
    assert!(!code.is_empty());

    // Test classic version
    app.version = "classic".to_string();
    let result = app.generate_code();
    assert!(result.is_ok());
    let code = result.unwrap();
    assert!(!code.is_empty());

    // Test with lowercase
    app.use_lowercase = true;
    let result = app.generate_code();
    assert!(result.is_ok());
}

#[test]
fn test_get_wa_configs() {
    let app = App::new();
    let (wa4, wa5) = app.get_wa_configs();

    assert_eq!(wa4, "ctdveirvrtidce");
    assert_eq!(wa5, "vridtcetvrdice");
}

#[test]
fn test_validate_code() {
    let mut app = App::new();
    app.battlenet_id = "TestUser#1234".to_string();

    // Generate a code and validate it
    let code = app.generate_code().unwrap();
    assert!(app.validate_code(&code));

    // Test with invalid base64
    assert!(!app.validate_code("invalid_base64!@#"));

    // Test with valid base64 but wrong content
    use base64::{engine::general_purpose, Engine as _};
    let wrong_content = general_purpose::STANDARD.encode("wrong_content");
    assert!(!app.validate_code(&wrong_content));
}

#[test]
fn test_default_implementation() {
    let app = App::default();
    assert_eq!(app.battlenet_id, "");
    assert!(!app.use_lowercase);
    assert_eq!(app.version, "retail");
    assert!(app.copy_feedback.is_none());
}

#[test]
fn test_toggle_version_with_invalid_version() {
    let mut app = App::new();
    app.version = "invalid_version".to_string();
    app.toggle_version();
    assert_eq!(app.version, "retail"); // Should default to retail
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

#[test]
fn test_validate_code_with_malformed_base64() {
    let app = App::new();

    // Test with string that contains invalid base64 characters
    assert!(!app.validate_code("invalid_base64_with_@!#$%"));

    // Test with empty string
    assert!(!app.validate_code(""));

    // Test with string that decodes but contains invalid UTF-8
    assert!(!app.validate_code("gA==")); // This decodes to invalid UTF-8
}

#[test]
fn test_validate_code_comprehensive_matching() {
    let mut app = App::new();
    app.battlenet_id = "TestUser#1234".to_string();

    // Test all possible combinations that should be valid
    app.version = "retail".to_string();

    // Test lowercase battlenet_id + config
    app.use_lowercase = true;
    let code = app.generate_code().unwrap();
    assert!(app.validate_code(&code));

    // Test original case battlenet_id + config
    app.use_lowercase = false;
    let code = app.generate_code().unwrap();
    assert!(app.validate_code(&code));
}

#[test]
fn test_decode_import_string_edge_cases() {
    let app = App::new();

    // Test decoding with whitespace and special characters (should be filtered)
    let test_input = "SGVsbG8gV29ybGQ="; // "Hello World" in base64
    let result = app.validate_code(&format!("  {test_input}  !@#"));
    // This should not crash and should handle the filtering
    assert!(!result); // It won't match our battlenet_id, but should not panic
}
