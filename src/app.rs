use crossterm::event::{KeyCode, KeyEvent};

pub struct AppState {
    pub running: bool,
}

impl AppState {
    pub fn new() -> Self {
        AppState { running: true }
    }
    pub fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.running = false,
            KeyCode::Char('p') => panic!("panic"),
            _ => {}
        }
    }
}
