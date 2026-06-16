use crossterm::event::KeyCode;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

use crate::{
    app::AppState,
    key_bindings::{self, KeyBinding},
    tui::state::UiState,
};

pub fn render(frame: &mut Frame, area: Rect, _app: &AppState, ui: &UiState) {
    let block = Block::bordered().title("Keybindings");
    let inner = block.inner(area);

    let [up, down] =
        Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]).areas(inner);

    let global_tips = Paragraph::new(binding_line(key_bindings::global_bindings())).centered();
    let section_tips =
        Paragraph::new(binding_line(key_bindings::bindings(&ui.focus))).centered();

    frame.render_widget(block, area);
    frame.render_widget(global_tips, up);
    frame.render_widget(section_tips, down);
}

fn key_label(key: &KeyCode) -> String {
    match key {
        KeyCode::Char(' ') => "Space".to_string(),
        KeyCode::Char(c) => c.to_uppercase().to_string(),
        KeyCode::Enter => "Enter".to_string(),
        KeyCode::Up => "↑".to_string(),
        KeyCode::Down => "↓".to_string(),
        KeyCode::Left => "←".to_string(),
        KeyCode::Right => "→".to_string(),
        KeyCode::Esc => "Esc".to_string(),
        KeyCode::Tab => "Tab".to_string(),
        KeyCode::Backspace => "Bksp".to_string(),
        KeyCode::Delete => "Del".to_string(),
        _ => format!("{key:?}"),
    }
}

fn binding_line(bindings: &[KeyBinding]) -> Line<'static> {
    let key_style = Style::default()
        .fg(Color::Yellow)
        .add_modifier(Modifier::BOLD);
    let sep_style = Style::default().fg(Color::DarkGray);

    let mut spans: Vec<Span<'static>> = Vec::new();
    for (i, b) in bindings.iter().enumerate() {
        if i > 0 {
            spans.push(Span::styled("  │  ", sep_style));
        }
        spans.push(Span::styled(format!("[{}]", key_label(&b.key)), key_style));
        spans.push(Span::raw(format!(" {}", b.description)));
    }
    Line::from(spans)
}
