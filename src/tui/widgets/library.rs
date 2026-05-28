use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, List},
};

use crate::{app::AppState, tui::state::UiState};

pub fn render(frame: &mut Frame, area: Rect, _app: &AppState, ui: &mut UiState) {
    let list = List::new(Vec::<String>::new()).block(Block::bordered().title("Library"));
    frame.render_stateful_widget(list, area, &mut ui.library_list);
}
