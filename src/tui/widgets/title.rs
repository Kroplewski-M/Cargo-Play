use ratatui::{Frame, layout::Rect, style::Stylize, text::Line, widgets::Paragraph};

use crate::app::AppState;

pub fn render(frame: &mut Frame, area: Rect, _app: &AppState) {
    let title = Paragraph::new(Line::from("cargo play").bold()).centered();
    frame.render_widget(title, area);
}
