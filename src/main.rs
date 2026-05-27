use crossterm::event::{self, Event};
use std::time::Duration;

use crate::app::AppState;
mod app;
mod error;
mod tui;
fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    tui::set_panic_hook();
    let result = run();
    tui::teardown()?;
    result
}

fn run() -> color_eyre::Result<()> {
    let mut terminal = tui::setup()?;
    let mut state = AppState::new();

    while state.running {
        terminal.draw(render)?;
        if event::poll(Duration::from_millis(16))?
            && let Event::Key(key) = event::read()?
        {
            state.handle_key(key);
        }
    }
    Ok(())
}

fn render(frame: &mut ratatui::Frame) {
    frame.render_widget("hello world", frame.area());
}
