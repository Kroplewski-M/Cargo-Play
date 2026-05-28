use crate::{app::AppState, tui::state::UiState};
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
};

mod library;
mod player_bar;
mod queue;

pub fn render(frame: &mut Frame, app: &AppState, ui: &mut UiState) {
    let [main, bar] =
        Layout::vertical([Constraint::Min(0), Constraint::Length(3)]).areas(frame.area());

    let [left, right] =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).areas(main);
    library::render(frame, left, app, ui);
    queue::render(frame, right, app, ui);
    player_bar::render(frame, bar, app);
}
