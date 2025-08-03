# SkillCapped Generator

[![Tests](https://github.com/Xerrion/skillcapped-generator/actions/workflows/tests.yml/badge.svg)](https://github.com/Xerrion/skillcapped-generator/actions/workflows/tests.yml)
[![codecov](https://codecov.io/gh/Xerrion/skillcapped-generator/branch/master/graph/badge.svg)](https://codecov.io/gh/Xerrion/skillcapped-generator)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=Xerrion_skillcapped-generator&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=Xerrion_skillcapped-generator)
[![Security Rating](https://sonarcloud.io/api/project_badges/measure?project=Xerrion_skillcapped-generator&metric=security_rating)](https://sonarcloud.io/summary/new_code?id=Xerrion_skillcapped-generator)
[![Coverage](https://sonarcloud.io/api/project_badges/measure?project=Xerrion_skillcapped-generator&metric=coverage)](https://sonarcloud.io/summary/new_code?id=Xerrion_skillcapped-generator)

A modern terminal-based unlock code generator for SkillCapped with beautiful UI and Battle.net ID validation.

## Features

- 🎮 **Version Support**: Both Classic and Retail WoW versions
- ✅ **Battle.net ID Validation**: Real-time validation with visual feedback
- 📋 **Clipboard Integration**: Copy unlock codes (Ctrl+C) and paste Battle.net IDs (Ctrl+V)
- 🎨 **Beautiful TUI**: Colorful and intuitive terminal interface with dedicated help section
- ⌨️ **Keyboard Shortcuts**: Full keyboard navigation and control
- 🔗 **GitHub Integration**: Quick access to project repository (Ctrl+G)

## Usage

1. Launch the application
2. Type or paste your Battle.net ID in the format: `Name#1234` (minimum 4 digits)
3. Use Tab to switch between Classic and Retail versions
4. Press Ctrl+C to copy the generated unlock code
5. Press Ctrl+V to paste a Battle.net ID from clipboard
6. Press Ctrl+G to open the GitHub repository
7. Press Ctrl+Q to quit

## Keyboard Shortcuts

- **Type/Paste**: Enter Battle.net ID (manual typing or Ctrl+V)
- **Tab**: Switch between Classic/Retail versions
- **Enter**: Toggle case sensitivity for unlock codes
- **Esc**: Clear input field
- **Ctrl+C**: Copy unlock code to clipboard
- **Ctrl+V**: Paste Battle.net ID from clipboard
- **Ctrl+G**: Open GitHub repository
- **Ctrl+Q**: Quit application

## Requirements

- Windows, macOS, or Linux
- Terminal with color support

## Installation

Download the latest release from GitHub or build from source with Rust/Cargo.

## Development

For information about setting up code quality tools (Codecov and SonarQube), see [docs/QUALITY_SETUP.md](docs/QUALITY_SETUP.md).

## Author

Made by **Xerrion** - [GitHub](https://github.com/Xerrion)

## License

MIT License - see LICENSE file for details.
