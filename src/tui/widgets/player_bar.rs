use ratatui::{Frame, layout::Rect};

use crate::{
    app::AppState,
    tui::{
        state::{Focus, UiState},
        widgets::Section,
    },
};

const SECTION: Section = Section(Focus::PlayerBar);

pub fn render(frame: &mut Frame, area: Rect, _app: &AppState, ui: &mut UiState) {
    let block = SECTION.block("Player control", ui);
    frame.render_widget(block, area);
}
