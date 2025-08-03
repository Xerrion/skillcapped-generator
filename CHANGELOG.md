# Changelog

All notable changes to the SkillCapped Generator project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.0.0] - 2025-08-03

### Added
- **Paste Functionality**: Added Ctrl+V keyboard shortcut to paste Battle.net IDs from clipboard
- **Dedicated Help Section**: New help section in the UI displaying all keyboard shortcuts with color-coded formatting
- **Enhanced Clipboard Integration**: Full bidirectional clipboard support for both copying unlock codes and pasting Battle.net IDs
- **Comprehensive Test Suite**: 41 unit and integration tests covering all functionality including edge cases
- **Modular Code Architecture**: Refactored codebase into organized modules (app, input, ui, terminal) for better maintainability
- **Input Sanitization**: Automatic cleanup of pasted content to ensure valid Battle.net ID format
- **Cross-platform Clipboard Support**: Using arboard crate for reliable clipboard operations across Windows, macOS, and Linux

### Changed
- **UI Layout**: Reorganized interface with 5 sections (Input, Version, Code Output, Help, Footer) for better information hierarchy
- **Input Field Design**: Simplified input field title, moved comprehensive help text to dedicated section
- **Help Text Organization**: Color-coded keyboard shortcuts with bold keys and descriptive text for improved readability
- **Code Structure**: Split monolithic main.rs (400+ lines) into focused, testable modules
- **Test Organization**: Centralized all tests in dedicated tests/ directory with comprehensive coverage

### Improved
- **User Experience**: Cleaner, more intuitive interface with logical information flow and visual organization
- **Performance**: Optimized rendering pipeline and efficient input handling with proper event management
- **Code Quality**: Enhanced maintainability through modularization, comprehensive testing, and clear separation of concerns
- **Documentation**: Updated README.md with complete feature documentation and usage instructions

### Technical Details
- Integrated `arboard 3.5.0` for cross-platform clipboard functionality
- Implemented `handle_paste_battlenet_id()` with input validation and sanitization
- Enhanced `draw_help_section()` with structured keyboard shortcut display
- Expanded test coverage from 0 to 41 tests with unit, integration, and UI testing
- Maintained backwards compatibility - all existing keyboard shortcuts remain functional

### Keyboard Shortcuts
- **Type/Paste**: Enter Battle.net ID (manual typing or Ctrl+V from clipboard)
- **Tab**: Switch between Classic/Retail WoW versions
- **Enter**: Toggle case sensitivity for unlock codes
- **Esc**: Clear input field
- **Ctrl+C**: Copy unlock code to clipboard
- **Ctrl+V**: Paste Battle.net ID from clipboard *(NEW)*
- **Ctrl+G**: Open GitHub repository in browser
- **Ctrl+Q**: Quit application
