use ratatui::widgets::ListState;

#[derive(Default)]
pub enum Focus {
    #[default]
    Library,
    Queue,
    PlayerBar,
}

#[derive(Default)]
pub struct UiState {
    pub library_list: ListState,
    pub queue_list: ListState,
    pub focus: Focus,
}

impl UiState {
    pub fn new() -> Self {
        Self {
            library_list: ListState::default(),
            queue_list: ListState::default(),
            focus: Focus::Library,
        }
    }
}
