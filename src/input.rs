use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    app::AppState,
    key_bindings::{Action, bindings, global_bindings},
    tui::state::{Focus, UiState},
};

pub fn handle_key(key: KeyEvent, app: &mut AppState, ui: &mut UiState) {
    let normalised = match key.code {
        KeyCode::Char(c) => KeyCode::Char(c.to_ascii_uppercase()),
        other => other,
    };

    let action = global_bindings()
        .iter()
        .chain(bindings(&ui.focus).iter())
        .find(|b| b.key == normalised)
        .map(|b| &b.action);

    if let Some(action) = action {
        match action {
            Action::Quit => app.running = false,
            Action::FocusSection(focus) => ui.focus = focus.clone(),
            Action::ScrollUp => scroll_up(app, ui),
            Action::ScrollDown => scroll_down(app, ui),
            Action::PlayPause => app.player_control.play_pause(),
            Action::PlayTrack => play_track(app, ui),
            _ => {}
        }
    }
}
fn scroll_up(app: &AppState, ui: &mut UiState) {
    match ui.focus {
        Focus::Library => {
            if !app.library.is_empty() {
                let next = ui
                    .library_list
                    .selected()
                    .map(|i| i.saturating_sub(1))
                    .unwrap_or(0);
                ui.library_list.select(Some(next));
            }
        }
        Focus::Queue => {}
        _ => {}
    }
}
fn scroll_down(app: &AppState, ui: &mut UiState) {
    match ui.focus {
        Focus::Library => {
            if !app.library.is_empty() {
                let next = ui
                    .library_list
                    .selected()
                    .map(|i| (i + 1).min(app.library.len() - 1))
                    .unwrap_or(0);
                ui.library_list.select(Some(next));
            }
        }
        Focus::Queue => {}
        _ => {}
    }
}
fn play_track(app: &mut AppState, ui: &UiState) {
    if let Some(index) = ui.library_list.selected()
        && let Some(track) = app.library.get(index)
    {
        app.player_control.play_track(track);
    }
}
