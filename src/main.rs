use crossterm::event::{self, Event};
use ratatui::style::Color;
use std::time::Duration;

use crate::{
    app::AppState,
    input::{handle_key, skip_track},
    tui::state::UiState,
};
mod app;
mod error;
mod input;
mod key_bindings;
mod models;
mod player;
mod scanner;
mod tui;

const MAIN_COLOR: Color = Color::Green;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    tui::set_panic_hook();
    let result = run();
    tui::teardown()?;
    Ok(result?)
}

fn run() -> error::Result<()> {
    let mut terminal = tui::setup()?;

    let mut app = AppState::new();
    let mut ui = UiState::new();
    app.library = scanner::scan(&dirs::audio_dir().unwrap_or_default())?;
    while app.running {
        terminal.draw(|f| tui::widgets::render(f, &app, &mut ui))?;
        if event::poll(Duration::from_millis(16))?
            && let Event::Key(key) = event::read()?
        {
            handle_key(key, &mut app, &mut ui);
        }

        if app.player_control.finished() {
            if app.player_control.looping()
                && let Some(track) = &app.player_control.current_track
            {
                app.player_control.play_track(&track.clone());
            } else {
                skip_track(&mut app);
            }
        }
    }
    Ok(())
}
