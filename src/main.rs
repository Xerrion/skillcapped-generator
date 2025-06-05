use base64::{Engine as _, engine::general_purpose};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io,
    time::{Duration, Instant},
};
use ratatui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style, Color},
    text::{Span, Line},
    widgets::{Block, Borders, Paragraph},
};
use arboard::Clipboard;

struct App {
    battlenet_id: String,
    use_lowercase: bool,
    version: String, // "classic" or "retail"
    last_input: Instant,
    copy_feedback: Option<Instant>, // For showing copy feedback
}

impl App {
    fn new() -> App {
        App {
            battlenet_id: String::new(),
            use_lowercase: false,
            version: "retail".to_string(),
            last_input: Instant::now(),
            copy_feedback: None,
        }
    }

    fn reset_input(&mut self) {
        self.battlenet_id.clear();
    }

    fn generate_code(&self) -> Result<String, String> {
        let (part1, part2) = match self.version.as_str() {
            "retail" => (
                vec![99, 116, 100, 118, 101, 105],           // "ctdvei"
                vec![114, 118, 114, 116, 105, 100, 99, 101], // "rvrtdice"
            ),
            "classic" => (
                vec![118, 114, 105, 100, 116, 99],           // "vridtc"
                vec![101, 116, 118, 114, 100, 105, 99, 101], // "etvrdice"
            ),
            _ => return Err("Invalid version".to_string()),
        };

        let part1_str: String = part1.into_iter().map(|c| c as u8 as char).collect();
        let part2_str: String = part2.into_iter().map(|c| c as u8 as char).collect();
        let addon_config = format!("{}{}", part1_str, part2_str);

        let mut input = self.battlenet_id.clone();
        if self.use_lowercase {
            input = input.to_lowercase();
        }
        input.push_str(&addon_config);

        Ok(general_purpose::STANDARD.encode(input))
    }

    fn sanitize_input(&mut self) {
        // Allow alphanumeric characters and # for Battle.net ID format (Name#Numbers)
        self.battlenet_id.retain(|c| c.is_ascii_alphanumeric() || c == '#');
    }

    fn is_valid_battlenet_id(&self) -> bool {
        // Check if the format is Name#Numbers (like Xerrion#2624)
        if self.battlenet_id.is_empty() {
            return false;
        }
        
        // Must contain exactly one # character
        let parts: Vec<&str> = self.battlenet_id.split('#').collect();
        if parts.len() != 2 {
            return false;
        }
        
        let name_part = parts[0];
        let number_part = parts[1];
        
        // Name part must not be empty and contain only alphanumeric characters
        if name_part.is_empty() || !name_part.chars().all(|c| c.is_ascii_alphanumeric()) {
            return false;
        }
        
        // Number part must be at least 4 digits long and contain only digits
        number_part.len() >= 4 && number_part.chars().all(|c| c.is_ascii_digit())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut app = App::new();
    // Make sure input is clear at startup
    app.reset_input();

    loop {
        // Sanitize input before displaying
        app.sanitize_input();
        
        terminal.draw(|f| {
            let size = f.size();

            let layout = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Min(1),
                    Constraint::Length(3),
                ])
                .split(size);

            let version_line = Line::from(vec![
                Span::styled(
                    if app.version == "classic" {
                        "● Classic ●"
                    } else {
                        "  Classic  "
                    },
                    Style::default()
                        .fg(if app.version == "classic" {
                            Color::Yellow
                        } else {
                            Color::Gray
                        })
                        .add_modifier(if app.version == "classic" {
                            Modifier::BOLD
                        } else {
                            Modifier::empty()
                        }),
                ),
                Span::styled(" | ", Style::default().fg(Color::Cyan)),
                Span::styled(
                    if app.version == "retail" {
                        "● Retail ●"
                    } else {
                        "  Retail  "
                    },
                    Style::default()
                        .fg(if app.version == "retail" {
                            Color::Yellow
                        } else {
                            Color::Gray
                        })
                        .add_modifier(if app.version == "retail" {
                            Modifier::BOLD
                        } else {
                            Modifier::empty()
                        }),
                ),
                Span::styled("    ⭳ Tab to switch", Style::default().fg(Color::Cyan)),
            ]);

            let (unlock_code, copy_title, code_color) = if app.is_valid_battlenet_id() {
                let code = app.generate_code().unwrap_or_else(|_| "Invalid version".to_string());
                let title = if let Some(copy_time) = app.copy_feedback {
                    if copy_time.elapsed() < Duration::from_secs(2) {
                        "🎉 Unlock Code (Copied to clipboard!)"
                    } else {
                        app.copy_feedback = None; // Reset feedback after 2 seconds
                        "🔑 Unlock Code (Ctrl+C to copy)"
                    } 
                } else {
                    "🔑 Unlock Code (Ctrl+C to copy)"
                };
                (code, title, Color::Green)
            } else {
                ("⚠️  Enter a valid Battle.net ID to generate unlock code".to_string(), "🔑 Unlock Code", Color::Yellow)
            };

            // Determine input validation styling
            let (input_style, input_border_color, status_emoji) = if app.battlenet_id.is_empty() {
                (Style::default().fg(Color::Cyan), Color::Cyan, "💭")
            } else if app.is_valid_battlenet_id() {
                (Style::default().fg(Color::Green), Color::Green, "✅")
            } else {
                (Style::default().fg(Color::Red), Color::Red, "❌")
            };

            f.render_widget(
                Paragraph::new(Line::from(vec![
                    Span::styled(status_emoji, Style::default()),
                    Span::styled(" Battle.net ID: ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                    Span::styled(
                        if app.battlenet_id.is_empty() {
                            "Type here... (format: Name#1234)"
                        } else {
                            &app.battlenet_id
                        },
                        input_style.add_modifier(Modifier::BOLD),
                    ),
                ]))
                .block(Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(input_border_color))
                    .title_style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))
                    .title("💻 Input (type to enter, Esc to clear, Ctrl+Q to quit)")),
                layout[0],
            );

            f.render_widget(
                Paragraph::new(version_line)
                    .block(Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Blue))
                        .title_style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))
                        .title("🎮 Version")),
                layout[1],
            );

            f.render_widget(
                Paragraph::new(Line::from(vec![
                    Span::styled(
                        unlock_code.clone(),
                        Style::default()
                            .fg(code_color)
                            .add_modifier(Modifier::BOLD)
                    )
                ]))
                .block(Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(code_color))
                    .title_style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))
                    .title(copy_title)),
                layout[2],
            );

            // Footer with credits
            f.render_widget(
                Paragraph::new(Line::from(vec![
                    Span::styled("Made by ", Style::default().fg(Color::Gray)),
                    Span::styled("Xerrion", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                    Span::styled(" | 🔗 ", Style::default().fg(Color::Gray)),
                    Span::styled("https://github.com/Xerrion", Style::default().fg(Color::Blue).add_modifier(Modifier::UNDERLINED).add_modifier(Modifier::BOLD)),
                    Span::styled(" (Ctrl+G to open)", Style::default().fg(Color::DarkGray)),
                ]))
                .block(Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::DarkGray))
                    .title_style(Style::default().fg(Color::DarkGray))
                    .title("ℹ️  About")),
                layout[3],
            );
        })?;

        if event::poll(Duration::from_millis(200))? {
            match event::read()? {
                Event::Key(key) => {
                    // Only handle key press events, ignore key release and repeat
                    if key.kind != KeyEventKind::Press {
                        continue;
                    }
                    
                    match key.code {
                        KeyCode::Char(c) if !key.modifiers.contains(KeyModifiers::CONTROL) && !key.modifiers.contains(KeyModifiers::ALT) && !key.modifiers.contains(KeyModifiers::SUPER) => {
                            app.battlenet_id.push(c);
                            app.last_input = Instant::now();
                        }
                        KeyCode::Backspace => {
                            app.battlenet_id.pop();
                        }
                        KeyCode::Tab => {
                            app.version = match app.version.as_str() {
                                "classic" => "retail".to_string(),
                                "retail" => "classic".to_string(),
                                _ => "retail".to_string(),
                            };
                        }
                        KeyCode::Enter => {
                            app.use_lowercase = !app.use_lowercase;
                        }
                        KeyCode::Esc => {
                            app.reset_input();
                        }
                        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            // Copy unlock code to clipboard only if input is valid
                            if app.is_valid_battlenet_id() {
                                if let Ok(unlock_code) = app.generate_code() {
                                    if let Ok(mut clipboard) = Clipboard::new() {
                                        if clipboard.set_text(unlock_code).is_ok() {
                                            app.copy_feedback = Some(Instant::now());
                                        }
                                    }
                                }
                            }
                        }
                        KeyCode::Char('g') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            // Open GitHub link in default browser
                            #[cfg(target_os = "windows")]
                            {
                                let _ = std::process::Command::new("cmd")
                                    .args(["/c", "start", "https://github.com/Xerrion"])
                                    .spawn();
                            }
                            #[cfg(target_os = "macos")]
                            {
                                let _ = std::process::Command::new("open")
                                    .arg("https://github.com/Xerrion")
                                    .spawn();
                            }
                            #[cfg(target_os = "linux")]
                            {
                                let _ = std::process::Command::new("xdg-open")
                                    .arg("https://github.com/Xerrion")
                                    .spawn();
                            }
                        }
                        KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            return Ok(())
                        }
                        _ => {}
                    }
                }
                Event::Mouse(_) => {
                    // Completely ignore mouse events without processing them
                }
                _ => {}
            }
        }

        // Update and clear copy feedback after 1 second
        if let Some(copy_instant) = app.copy_feedback {
            if Instant::now().duration_since(copy_instant).as_secs() >= 1 {
                app.copy_feedback = None;
            }
        }
    }
}
