use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    widgets::Paragraph,
};

use crate::{
    app::AppState,
    tui::{
        state::{Focus, UiState},
        widgets::Section,
    },
};

const SECTION: Section = Section(Focus::PlayerBar);

pub fn render(frame: &mut Frame, area: Rect, app: &AppState, ui: &mut UiState) {
    let block = SECTION.block("Player Control(3)", ui);
    let inner = block.inner(area);
    let [up, down] =
        Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]).areas(inner);
    if let Some(track) = &app.player_control.current_track {
        let playing = if app.player_control.is_paused() {
            "paused"
        } else {
            "playing"
        };
        let p = Paragraph::new(format!("currently {}: {}", playing, track.name));
        frame.render_widget(p, up);
    }
    let vol = Paragraph::new(format!(
        "volume: {:0}%",
        (app.player_control.volume() * 100.0) as u32
    ));
    frame.render_widget(vol, down);
    frame.render_widget(block, area);
}
