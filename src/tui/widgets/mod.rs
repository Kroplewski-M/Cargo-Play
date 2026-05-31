use crate::{
    app::AppState,
    tui::state::{Focus, UiState},
};
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Style},
    widgets::Block,
};

mod library;
mod player_bar;
mod queue;
mod tool_tip;

pub struct Section(pub Focus);

impl Section {
    pub fn is_active(&self, ui: &UiState) -> bool {
        ui.focus == self.0
    }
    pub fn block<'a>(&self, title: &'a str, ui: &UiState) -> Block<'a> {
        let border_style = if self.is_active(ui) {
            Style::new().fg(Color::Green)
        } else {
            Style::default()
        };
        Block::bordered().title(title).border_style(border_style)
    }
}

pub fn render(frame: &mut Frame, app: &AppState, ui: &mut UiState) {
    let [tooltip, main, player] = Layout::vertical([
        Constraint::Length(2),
        Constraint::Min(0),
        Constraint::Length(6),
    ])
    .areas(frame.area());

    let [left, right] =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).areas(main);
    library::render(frame, left, app, ui);
    queue::render(frame, right, app, ui);
    player_bar::render(frame, player, app, ui);
    tool_tip::render(frame, tooltip, app);
}
