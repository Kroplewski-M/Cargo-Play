use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    app::AppState,
    key_bindings::{Action, bindings, global_bindings},
    tui::state::UiState,
};

pub fn handle_key(key: KeyEvent, app: &mut AppState, ui: &mut UiState) {
    let normalised = match key.code {
        KeyCode::Char(c) => KeyCode::Char(c.to_ascii_uppercase()),
        other => other,
    };

    let map: HashMap<KeyCode, &Action> = global_bindings()
        .iter()
        .chain(bindings(&ui.focus).iter())
        .map(|b| (b.key, &b.action))
        .collect();

    if let Some(action) = map.get(&normalised) {
        match action {
            Action::Quit => app.running = false,
            Action::FocusSection(focus) => ui.focus = focus.clone(),
            Action::AddTrackToLibrary => {}
            _ => {}
        }
    }
}
