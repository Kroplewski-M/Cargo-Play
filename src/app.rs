use crate::models::Track;

pub struct AppState {
    pub running: bool,
    pub library: Vec<Track>,
}
impl Default for AppState {
    fn default() -> Self {
        Self {
            running: true,
            library: vec![],
        }
    }
}
impl AppState {
    pub fn new() -> Self {
        Self {
            running: true,
            library: vec![],
        }
    }
}
