use crossterm::event::{KeyCode, KeyEvent};

use crate::tui::state::{Focus, UiState};

pub struct AppState {
    pub running: bool,
}

impl AppState {
    pub fn new() -> Self {
        AppState { running: true }
    }
    pub fn handle_key(&mut self, key: KeyEvent, ui: &mut UiState) {
        match key.code {
            KeyCode::Char('q') => self.running = false,
            KeyCode::Char('p') => panic!("panic"),
            KeyCode::Char('1') => ui.focus = Focus::Library,
            KeyCode::Char('2') => ui.focus = Focus::Queue,
            KeyCode::Char('3') => ui.focus = Focus::PlayerBar,
            _ => {}
        }
    }
}
