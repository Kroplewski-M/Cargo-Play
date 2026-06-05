use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::Style,
    widgets::{Cell, Padding, Row, Table},
};

use crate::{
    app::AppState,
    tui::{
        state::{Focus, UiState},
        widgets::Section,
    },
};

const SECTION: Section = Section(Focus::Library);

pub fn render(frame: &mut Frame, area: Rect, app: &AppState, ui: &mut UiState) {
    let rows: Vec<Row> = app
        .library
        .iter()
        .map(|t| {
            Row::new(vec![
                Cell::from(t.name.as_str()),
                Cell::from(t.formatted_duration()),
            ])
        })
        .collect();

    let table = Table::new(rows, [Constraint::Fill(1), Constraint::Length(8)])
        .block(
            SECTION
                .block("Library(1)", ui)
                .padding(Padding::horizontal(1)),
        )
        .header(
            Row::new(vec!["Name", "Duration"])
                .style(Style::new().bold().underlined())
                .bottom_margin(1),
        )
        .row_highlight_style(Style::new().on_green().black());

    frame.render_stateful_widget(table, area, &mut ui.library_list);
}
