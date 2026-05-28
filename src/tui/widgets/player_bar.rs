use ratatui::{Frame, layout::Rect, widgets::Block};

use crate::app::AppState;

pub fn render(frame: &mut Frame, area: Rect, _app: &AppState) {
    let block = Block::bordered().title("player");
    frame.render_widget(block, area);
}
