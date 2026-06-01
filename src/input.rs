use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    app::AppState,
    tui::state::{Focus, UiState},
};

pub fn handle_key(key: KeyEvent, app: &mut AppState, ui: &mut UiState) {
    match key.code {
        KeyCode::Char('q') => app.running = false,
        KeyCode::Char('1') => ui.focus = Focus::Library,
        KeyCode::Char('2') => ui.focus = Focus::Queue,
        KeyCode::Char('3') => ui.focus = Focus::PlayerBar,
        _ => {}
    }
    match ui.focus {
        Focus::Library => handle_library_key(key, app, ui),
        Focus::Queue => handle_queue_key(key, app, ui),
        Focus::PlayerBar => handle_player_key(key, app, ui),
    }
}

fn handle_library_key(key: KeyEvent, app: &mut AppState, ui: &mut UiState) {}

fn handle_queue_key(key: KeyEvent, app: &mut AppState, ui: &mut UiState) {}
fn handle_player_key(key: KeyEvent, app: &mut AppState, ui: &mut UiState) {}
