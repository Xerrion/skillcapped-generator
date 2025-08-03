use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use std::time::{Duration, Instant};

pub fn draw_ui(f: &mut Frame, app: &App) {
    let size = f.area();

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3), // Input
            Constraint::Length(3), // Version
            Constraint::Min(1),    // Code output
            Constraint::Length(4), // Help
            Constraint::Length(3), // Footer
        ])
        .split(size);

    draw_input_section(f, app, layout[0]);
    draw_version_section(f, app, layout[1]);
    draw_code_section(f, app, layout[2]);
    draw_help_section(f, layout[3]);
    draw_footer_section(f, layout[4]);
}

fn draw_input_section(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let (input_style, input_border_color, status_emoji) = get_input_styling(app);
    let input_text = get_input_display_text(app);
    let input_spans = create_input_spans(status_emoji, input_text, input_style);

    f.render_widget(
        Paragraph::new(Line::from(input_spans)).block(create_input_block(input_border_color)),
        area,
    );
}

fn get_input_display_text(app: &App) -> &str {
    if app.battlenet_id.is_empty() {
        "Type here... (format: Name#1234)"
    } else {
        &app.battlenet_id
    }
}

fn create_input_spans<'a>(
    status_emoji: &'static str,
    input_text: &'a str,
    input_style: Style,
) -> Vec<Span<'a>> {
    vec![
        Span::styled(status_emoji, Style::default()),
        Span::styled(
            " Battle.net ID: ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(input_text, input_style.add_modifier(Modifier::BOLD)),
    ]
}

fn create_input_block(border_color: Color) -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color))
        .title_style(
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        )
        .title("💻 Input")
}

fn draw_version_section(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let version_line = Line::from(vec![
        create_version_span("Classic", &app.version, "classic"),
        Span::styled(" | ", Style::default().fg(Color::Cyan)),
        create_version_span("Retail", &app.version, "retail"),
        Span::styled("    ⭳ Tab to switch", Style::default().fg(Color::Cyan)),
    ]);

    f.render_widget(
        Paragraph::new(version_line).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Blue))
                .title_style(
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD),
                )
                .title("🎮 Version"),
        ),
        area,
    );
}

fn draw_code_section(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let (unlock_code, copy_title, code_color) = get_code_info(app);

    f.render_widget(
        Paragraph::new(Line::from(vec![Span::styled(
            unlock_code,
            Style::default().fg(code_color).add_modifier(Modifier::BOLD),
        )]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(code_color))
                .title_style(
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD),
                )
                .title(copy_title),
        ),
        area,
    );
}

fn draw_help_section(f: &mut Frame, area: ratatui::layout::Rect) {
    let help_spans = create_help_spans();

    f.render_widget(
        Paragraph::new(Line::from(help_spans)).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow))
                .title_style(
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD),
                )
                .title("❓ Help"),
        ),
        area,
    );
}

fn create_help_spans() -> Vec<Span<'static>> {
    let help_items = [
        ("Type/Paste: ", "Enter Battle.net ID"),
        ("Esc: ", "Clear"),
        ("Ctrl+C: ", "Copy"),
        ("Ctrl+V: ", "Paste"),
        ("Ctrl+Q: ", "Quit"),
    ];

    let mut spans = vec![Span::styled("⌨️  ", Style::default().fg(Color::Yellow))];

    for (i, (command, description)) in help_items.iter().enumerate() {
        if i > 0 {
            spans.push(Span::styled(" | ", Style::default().fg(Color::DarkGray)));
        }

        spans.push(Span::styled(
            *command,
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ));
        spans.push(Span::styled(*description, Style::default().fg(Color::Gray)));
    }

    spans
}

fn draw_footer_section(f: &mut Frame, area: ratatui::layout::Rect) {
    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled("Made by ", Style::default().fg(Color::Gray)),
            Span::styled(
                "Xerrion",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" | 🔗 ", Style::default().fg(Color::Gray)),
            Span::styled(
                "https://github.com/Xerrion",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::UNDERLINED)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" (Ctrl+G to open)", Style::default().fg(Color::DarkGray)),
        ]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray))
                .title_style(Style::default().fg(Color::DarkGray))
                .title("ℹ️  About"),
        ),
        area,
    );
}

fn get_input_styling(app: &App) -> (Style, Color, &'static str) {
    match (app.battlenet_id.is_empty(), app.is_valid_battlenet_id()) {
        (true, _) => (Style::default().fg(Color::Cyan), Color::Cyan, "💭"),
        (false, true) => (Style::default().fg(Color::Green), Color::Green, "✅"),
        (false, false) => (Style::default().fg(Color::Red), Color::Red, "❌"),
    }
}

fn create_version_span<'a>(name: &'a str, current_version: &str, version_key: &str) -> Span<'a> {
    let is_current = current_version == version_key;
    let text = if is_current {
        format!("● {name} ●")
    } else {
        format!("  {name}  ")
    };

    Span::styled(
        text,
        Style::default()
            .fg(if is_current {
                Color::Yellow
            } else {
                Color::Gray
            })
            .add_modifier(if is_current {
                Modifier::BOLD
            } else {
                Modifier::empty()
            }),
    )
}

fn get_code_info(app: &App) -> (String, &'static str, Color) {
    match app.is_valid_battlenet_id() {
        true => get_valid_code_info(app),
        false => get_invalid_code_info(),
    }
}

fn get_valid_code_info(app: &App) -> (String, &'static str, Color) {
    let code = app
        .generate_code()
        .unwrap_or_else(|_| "Invalid version".to_string());

    let title = get_copy_feedback_title(app.copy_feedback);
    (code, title, Color::Green)
}

fn get_invalid_code_info() -> (String, &'static str, Color) {
    (
        "⚠️  Enter a valid Battle.net ID to generate unlock code".to_string(),
        "🔑 Unlock Code",
        Color::Yellow,
    )
}

fn get_copy_feedback_title(copy_feedback: Option<Instant>) -> &'static str {
    copy_feedback
        .filter(|&copy_time| copy_time.elapsed() < Duration::from_secs(2))
        .map(|_| "🎉 Unlock Code (Copied to clipboard!)")
        .unwrap_or("🔑 Unlock Code (Ctrl+C to copy)")
}
