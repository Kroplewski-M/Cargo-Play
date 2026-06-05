use ratatui::widgets::TableState;

#[derive(Default, PartialEq, Clone)]
pub enum Focus {
    #[default]
    Library,
    Queue,
    PlayerBar,
}

#[derive(Default)]
pub struct UiState {
    pub library_list: TableState,
    pub queue_list: TableState,
    pub focus: Focus,
}

impl UiState {
    pub fn new() -> Self {
        Self {
            library_list: TableState::default(),
            queue_list: TableState::default(),
            focus: Focus::Library,
        }
    }
}
