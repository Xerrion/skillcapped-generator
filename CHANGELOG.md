# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [2.1.0] - 2025-08-03

### Changed
- Reduced cyclomatic complexity across multiple modules for better maintainability
- Refactored `app.rs` validation and code generation functions using functional programming patterns
- Simplified event handling logic in `input.rs` with cleaner control flow
- Improved UI styling functions in `ui.rs` using pattern matching instead of nested conditionals
- Enhanced terminal event processing in `terminal.rs` with more functional approaches
- Extracted magic numbers into named constants for better code clarity
- Replaced platform-specific conditional compilation with runtime detection

### Technical Improvements
- Applied functional programming principles using `map`, `filter`, `and_then` operations
- Reduced nested conditionals and branching complexity
- Improved code readability and maintainability
- Enhanced separation of concerns with smaller, focused functions

## [2.0.1] - 2025-08-03

### Added
- Command line interface with options for Battle.net ID input, clipboard copying, and quiet mode
- Windows executable metadata including version information, company details, and manifest
- Application icon embedded in Windows executable
- Help system displaying all available command line options

### Changed
- Build system simplified to use only embed-resource for Windows resource compilation
- Copyright symbol encoding fixed in Windows metadata from "©" to "(C)"

### Fixed
- Character encoding issues in Windows executable metadata

## [2.0.0] - 2025-08-03

### Added
- Paste functionality with Ctrl+V keyboard shortcut for Battle.net IDs from clipboard
- Dedicated help section in UI displaying all keyboard shortcuts with color-coded formatting
- Comprehensive test suite with 41 unit and integration tests
- Input sanitization for pasted content to ensure valid Battle.net ID format
- Cross-platform clipboard support using arboard crate
- Modular code architecture with organized modules (app, input, ui, terminal)

### Changed
- UI layout reorganized with 5 sections (Input, Version, Code Output, Help, Footer)
- Enhanced user experience with real-time feedback and visual improvements
- Code structure refactored for better maintainability and testing

### Fixed
- Improved error handling for clipboard operations
- Better input validation and user feedback

## [1.0.0] - 2025-08-02

### Added
- Initial release of SkillCapped Generator
- Terminal-based user interface for generating unlock codes
- Battle.net ID validation with format checking
- Base64 encoding for unlock code generation
- Copy to clipboard functionality with Ctrl+C
- Version switching between retail and classic WoW
- Real-time input validation and feedback

[unreleased]: https://github.com/Xerrion/skillcapped-generator/compare/v2.1.0...HEAD
[2.1.0]: https://github.com/Xerrion/skillcapped-generator/compare/v2.0.1...v2.1.0
[2.0.1]: https://github.com/Xerrion/skillcapped-generator/compare/v2.0.0...v2.0.1
[2.0.0]: https://github.com/Xerrion/skillcapped-generator/compare/v1.0.0...v2.0.0
[1.0.0]: https://github.com/Xerrion/skillcapped-generator/releases/tag/v1.0.0
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
