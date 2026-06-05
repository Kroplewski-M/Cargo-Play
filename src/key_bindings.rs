use crossterm::event::KeyCode;

use crate::tui::state::Focus;

pub enum Action {
    FocusSection(Focus),
    ScrollUp,
    ScrollDown,
    AddTrackToLibrary,
    DeleteTrackFromLibrary,
    Quit,
}

pub struct KeyBinding {
    pub key: KeyCode,
    pub description: &'static str,
    pub action: Action,
}

pub fn bindings(focus: &Focus) -> &'static [KeyBinding] {
    match focus {
        Focus::Library => &[
            KeyBinding {
                key: KeyCode::Char('J'),
                description: "Scroll Down",
                action: Action::ScrollDown,
            },
            KeyBinding {
                key: KeyCode::Char('K'),
                description: "Scroll Up",
                action: Action::ScrollUp,
            },
        ],
        Focus::Queue => &[],
        Focus::PlayerBar => &[],
    }
}
pub fn global_bindings() -> &'static [KeyBinding] {
    &[
        KeyBinding {
            key: KeyCode::Char('Q'),
            description: "Quit",
            action: Action::Quit,
        },
        KeyBinding {
            key: KeyCode::Char('1'),
            description: "Focus library",
            action: Action::FocusSection(Focus::Library),
        },
        KeyBinding {
            key: KeyCode::Char('2'),
            description: "Focus queue",
            action: Action::FocusSection(Focus::Queue),
        },
        KeyBinding {
            key: KeyCode::Char('3'),
            description: "Focus player control",
            action: Action::FocusSection(Focus::PlayerBar),
        },
    ]
}
