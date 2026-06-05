use ratatui::{Frame, layout::Rect, widgets::Table};

use crate::{
    app::AppState,
    tui::{
        state::{Focus, UiState},
        widgets::Section,
    },
};
const SECTION: Section = Section(Focus::Queue);
pub fn render(frame: &mut Frame, area: Rect, _app: &AppState, ui: &mut UiState) {
    let list = Table::default().block(SECTION.block("Queue(2)", ui));
    frame.render_stateful_widget(list, area, &mut ui.queue_list);
}
