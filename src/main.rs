pub mod app;
pub mod input;
pub mod terminal;
pub mod ui;

use clap::{Arg, Command};
use terminal::{restore_terminal, run_app, setup_terminal};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("skillcapped-generator")
        .version("2.0.1")
        .author("Xerrion <https://github.com/Xerrion>")
        .about("SkillCapped Unlock Code Generator - A terminal-based tool for generating unlock codes for SkillCapped with Battle.net ID validation")
        .long_about("A powerful terminal-based utility for generating SkillCapped unlock codes with integrated Battle.net ID validation. \
                     This tool provides a user-friendly interface for creating and copying unlock codes directly to your clipboard.")
        .arg(Arg::new("battlenet-id")
            .short('b')
            .long("battlenet-id")
            .value_name("ID")
            .help("Generate unlock code for the specified Battle.net ID")
            .long_help("Provide a Battle.net ID to generate an unlock code directly without using the interactive interface"))
        .arg(Arg::new("copy")
            .short('c')
            .long("copy")
            .action(clap::ArgAction::SetTrue)
            .help("Automatically copy the generated code to clipboard")
            .long_help("When used with --battlenet-id, automatically copies the generated unlock code to the system clipboard"))
        .arg(Arg::new("quiet")
            .short('q')
            .long("quiet")
            .action(clap::ArgAction::SetTrue)
            .help("Run in quiet mode (minimal output)")
            .long_help("Suppress all output except the generated unlock code. Useful for scripting or automation"))
        .get_matches();

    // Handle command line arguments
    if let Some(battlenet_id) = matches.get_one::<String>("battlenet-id") {
        return handle_cli_mode(
            battlenet_id,
            matches.get_flag("copy"),
            matches.get_flag("quiet"),
        );
    }

    // No arguments provided, run interactive TUI
    if !matches.get_flag("quiet") {
        println!("Starting SkillCapped Generator interactive mode...");
        println!("Use --help for command line options.\n");
    }

    let mut terminal = setup_terminal()?;
    let res = run_app(&mut terminal);
    restore_terminal(&mut terminal)?;

    if let Err(err) = res {
        println!("Error: {err:?}");
    }

    Ok(())
}

fn handle_cli_mode(
    battlenet_id: &str,
    copy_to_clipboard: bool,
    quiet: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    use crate::app::App;

    if !quiet {
        println!("Generating unlock code for Battle.net ID: {}", battlenet_id);
    }

    // Use the same validation logic as the TUI
    let mut app = App::new();

    // Set the battlenet_id and validate
    app.battlenet_id = battlenet_id.to_string();

    // Validate the Battle.net ID
    if !app.is_valid_battlenet_id() {
        let error_msg =
            "Invalid Battle.net ID format. Expected format: Name#1234 (at least 4 digits)";
        if !quiet {
            eprintln!("Error: {}", error_msg);
        } else {
            eprintln!("{}", error_msg);
        }
        std::process::exit(1);
    }

    // Generate the unlock code
    let unlock_code = match app.generate_code() {
        Ok(code) => code,
        Err(err) => {
            if !quiet {
                eprintln!("Error generating unlock code: {}", err);
            } else {
                eprintln!("{}", err);
            }
            std::process::exit(1);
        }
    };

    // Copy to clipboard if requested
    if copy_to_clipboard {
        match arboard::Clipboard::new() {
            Ok(mut clipboard) => {
                if let Err(e) = clipboard.set_text(&unlock_code) {
                    if !quiet {
                        eprintln!("Warning: Failed to copy to clipboard: {}", e);
                    }
                } else if !quiet {
                    println!("Unlock code copied to clipboard!");
                }
            }
            Err(e) => {
                if !quiet {
                    eprintln!("Warning: Failed to access clipboard: {}", e);
                }
            }
        }
    }

    // Output the unlock code
    if quiet {
        println!("{}", unlock_code);
    } else {
        println!("Generated unlock code: {}", unlock_code);
        if !copy_to_clipboard {
            println!("\nUse this code in SkillCapped to unlock premium features.");
            println!("Tip: Use -c or --copy to automatically copy to clipboard.");
        } else {
            println!("\nUse this code in SkillCapped to unlock premium features.");
        }
    }

    Ok(())
}
