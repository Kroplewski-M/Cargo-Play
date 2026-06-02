use ratatui::{Frame, layout::Rect, widgets::List};

use crate::{
    app::AppState,
    tui::{
        state::{Focus, UiState},
        widgets::Section,
    },
};

const SECTION: Section = Section(Focus::Library);

pub fn render(frame: &mut Frame, area: Rect, _app: &AppState, ui: &mut UiState) {
    let list = List::new(Vec::<String>::new()).block(SECTION.block("Library(1)", ui));
    frame.render_stateful_widget(list, area, &mut ui.library_list);
}
