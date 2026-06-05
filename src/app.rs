use crate::models::Track;

pub struct AppState {
    pub running: bool,
    pub library: Vec<Track>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            running: true,
            library: vec![],
        }
    }
}
