use crossterm::event::{self, Event};
use std::time::Duration;

use crate::{app::AppState, db::Database, input::handle_key, tui::state::UiState};
mod app;
mod db;
mod error;
mod input;
mod key_bindings;
mod models;
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
    let database = open_sql_connection()?;

    let mut app = AppState::new();
    app.library = db::queries::get_tracks(&database)?;
    let mut ui = UiState::new();
    while app.running {
        terminal.draw(|f| tui::widgets::render(f, &app, &mut ui))?;
        if event::poll(Duration::from_millis(16))?
            && let Event::Key(key) = event::read()?
        {
            handle_key(key, &mut app, &mut ui);
        }
    }
    Ok(())
}

fn open_sql_connection() -> error::Result<Database> {
    let db_path = dirs::data_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("cargo_play")
        .join("cargo_play.db");
    std::fs::create_dir_all(db_path.parent().unwrap())?;
    Database::open(&db_path)
}
