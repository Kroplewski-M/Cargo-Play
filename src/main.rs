use crossterm::event::{self, Event};
use std::time::Duration;

use crate::{app::AppState, tui::state::UiState};
mod app;
mod error;
mod tui;
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

    while app.running {
        terminal.draw(|f| tui::widgets::render(f, &app, &mut ui))?;
        if event::poll(Duration::from_millis(16))?
            && let Event::Key(key) = event::read()?
        {
            app.handle_key(key);
        }
    }
    Ok(())
}
