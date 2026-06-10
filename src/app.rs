use crate::{models::Track, player::PlayerControl};

pub struct AppState {
    pub running: bool,
    pub library: Vec<Track>,
    pub queue: Vec<Track>,
    pub player_control: PlayerControl,
}
impl Default for AppState {
    fn default() -> Self {
        Self {
            running: true,
            library: vec![],
            queue: vec![],
            player_control: PlayerControl::new(),
        }
    }
}
impl AppState {
    pub fn new() -> Self {
        Self::default()
    }
}
