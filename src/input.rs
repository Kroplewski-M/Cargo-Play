use std::collections::HashMap;

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

    let map: HashMap<KeyCode, &Action> = global_bindings()
        .iter()
        .chain(bindings(&ui.focus).iter())
        .map(|b| (b.key, &b.action))
        .collect();

    if let Some(action) = map.get(&normalised) {
        match action {
            Action::Quit => app.running = false,
            Action::FocusSection(focus) => ui.focus = focus.clone(),
            Action::ScrollUp => scroll_up(app, ui),
            Action::ScrollDown => scroll_down(app, ui),
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
