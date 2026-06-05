use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Paragraph},
};

use crate::{
    app::AppState,
    key_bindings::{self, KeyBinding},
    tui::state::UiState,
};

pub fn render(frame: &mut Frame, area: Rect, _app: &AppState, ui: &UiState) {
    let block = Block::bordered().title("Tool Tip");

    let inner = block.inner(area);
    let [up, down] =
        Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]).areas(inner);
    let section_tips = Paragraph::new(get_bindings(key_bindings::bindings(&ui.focus))).centered();
    let global_tips = Paragraph::new(get_bindings(key_bindings::global_bindings())).centered();

    frame.render_widget(block, area);
    frame.render_widget(global_tips, up);
    frame.render_widget(section_tips, down);
}

fn get_bindings(bindings: &[KeyBinding]) -> String {
    bindings
        .iter()
        .map(|b| format!("{} ({})", b.key, b.description))
        .collect::<Vec<String>>()
        .join(" | ")
}
