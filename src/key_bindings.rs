use crossterm::event::KeyCode;

use crate::tui::state::Focus;

pub enum Action {
    FocusSection(Focus),
    ScrollUp,
    ScrollDown,
    PlayPause,
    PlayTrack,
    VolumeUp,
    VolumeDown,
    AddToQueue,
    RemoveFromQueue,
    LoopTrack,
    SkipTrack,
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
            KeyBinding {
                key: KeyCode::Char(' '),
                description: "Play Track",
                action: Action::PlayTrack,
            },
            KeyBinding {
                key: KeyCode::Char('A'),
                description: "Add Track To Queue",
                action: Action::AddToQueue,
            },
        ],
        Focus::Queue => &[
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
            KeyBinding {
                key: KeyCode::Char('D'),
                description: "Remove From Queue",
                action: Action::RemoveFromQueue,
            },
        ],
        Focus::PlayerBar => &[KeyBinding {
            key: KeyCode::Char('L'),
            description: "Loop Track",
            action: Action::LoopTrack,
        }],
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
        KeyBinding {
            key: KeyCode::Up,
            description: "Volume Up",
            action: Action::VolumeUp,
        },
        KeyBinding {
            key: KeyCode::Down,
            description: "Volume Down",
            action: Action::VolumeDown,
        },
        KeyBinding {
            key: KeyCode::Enter,
            description: "Play/Pause Track",
            action: Action::PlayPause,
        },
        KeyBinding {
            key: KeyCode::Char('S'),
            description: "Skip Track",
            action: Action::SkipTrack,
        },
    ]
}
